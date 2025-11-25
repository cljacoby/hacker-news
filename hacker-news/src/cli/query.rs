use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use futures::pin_mut;
use futures::stream::StreamExt;
use tracing::{error, info};

use crate::cli::HnCommand;
use crate::client::CommentNode;
use crate::client::HnClient;
use crate::error::HnError;
use crate::model::Id;

pub struct Query;

impl Query {
    /// Format a [CommentNode] for printing in the terminal as an ASCII thread.
    fn fmt_comment(cnode: &CommentNode, tree_mode: bool) -> String {
        let author = cnode.comment.by.as_ref().map(|s| s.as_str()).unwrap_or("");

        let indent: String = vec!["  "; cnode.depth].into_iter().collect();

        if tree_mode {
            format!("{}{}|{}", indent, author, cnode.comment.id)
        } else {
            let text = cnode
                .comment
                .text
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("");
            format!("{}{}|{}|{}", indent, author, cnode.comment.id, text)
        }
    }
}

impl HnCommand for Query {
    const NAME: &'static str = "query";

    fn parser<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
        .arg(
            Arg::with_name("id")
                .value_name("id")
                .required(true)
                .takes_value(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("lazy")
                .long("lazy")
                .short("l")
                .help("Lazily fetch the comment tree, and stream the results to the terminal")
                .required(false)
                .takes_value(false)
        )
        .arg(
            Arg::with_name("tree")
                .long("tree")
                .short("t")
                .help("Only print the comment_id and author, and not the comment text. Useful for getting a unix tree like output")
                .required(false)
                .takes_value(false)
        )
    }

    async fn cmd(matches: &ArgMatches<'_>) -> Result<(), Box<HnError>> {
        // SAFE: The clap App will guarantee required arguments are received
        let id = matches
            .value_of("id")
            .expect("clap failed to produce default value");
        let id: Id = id
            .parse()
            .map_err(|_| HnError::ArgumentError(Some("id argument not parseable as numeric")))?;

        let client = HnClient::new();
        let lazy = matches.is_present("lazy");
        let tree_mode = matches.is_present("tree");
        info!(lazy=?lazy, tree_mode=?tree_mode, "query cmd");

        // todo:
        //  - attempt to combine the two approaches below
        //  - use a separate thread to initiate I/O, but allows the stream.next()
        //    calls to yield comments results as they become available

        if lazy {
            // - lazily fetches comments while walking the thread
            // - requires the calls to stream.next() to actually initiate new request I/O
            // - slower to get the entire thread, but faster to yield initial results.
            let thread = client
                .lazy_thread(id)
                .await
                .expect("failed to create lazy_thread");
            let stream = thread.walk();
            pin_mut!(stream);
            while let Some(result) = stream.next().await {
                match result {
                    Ok(cnode) => {
                        let s = Self::fmt_comment(&cnode, tree_mode);
                        println!("{}", s);
                    }
                    Err(e) => {
                        error!("error fetching comment, will retry later: {}", e);
                    }
                }
            }
        } else {
            // - pre-fetches all comments before the walk() call
            // - initiates as many request I/Os as possible to maximize concurrency
            // - faster to get all comments, but only gives first comment once all are loaded.
            let thread = client.thread(id).await.expect("failed to create thread");
            for cnode in thread.walk() {
                let s = Self::fmt_comment(&cnode, tree_mode);
                println!("{}", s);
            }
        }

        Ok(())
    }
}
