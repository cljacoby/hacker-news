use std::io::Write;
use env_logger;
use env_logger::Builder;
use log;
use std::error::Error;
use log::LevelFilter;
use scraper::Html;
use scraper::Selector;
use scraper::element_ref::ElementRef;

pub mod error;
pub mod client;
pub mod config;
pub mod parse;
pub mod models;

pub fn init_logger() {
    let mut logger = Builder::from_default_env();
    logger.filter(Some("hnews::html"), LevelFilter::Trace);
    logger.format(|buf, record| {
        
        let timestamp = buf.timestamp();
        let level = record.level();
        let mod_path = record.module_path().unwrap_or("Could not obtain module path");
        let file =record.file().unwrap_or("Could not obtain file");
        let line = record.line().unwrap_or(0);
        let args = record.args();

        writeln!(
            buf,
            "[{timestamp} {level} {mod_path} {file}:{line}] {args}",
            timestamp = timestamp,
            level = level,
            mod_path = mod_path,
            file = file,
            line = line,
            args = args
        )
    });

    logger.init();
}

pub fn get_test_text() -> String {
    String::from(r###"
    <html lang="en" op="item"><head><meta name="referrer" content="origin"><meta name="viewport" content="width=device-width, initial-scale=1.0"><link rel="stylesheet" type="text/css" href="news.css?t6cz9VDBRNMVBhMCuz1j">
            <link rel="shortcut icon" href="favicon.ico">
            <title>Vulnerability allows cross-browser tracking in Chrome, Firefox, Safari, and Tor | Hacker News</title></head><body><center><table id="hnmain" border="0" cellpadding="0" cellspacing="0" width="85%" bgcolor="#f6f6ef">
            <tr><td bgcolor="#ff6600"><table border="0" cellpadding="0" cellspacing="0" width="100%" style="padding:2px"><tr><td style="width:18px;padding-right:4px"><a href="https://news.ycombinator.com"><img src="y18.gif" width="18" height="18" style="border:1px white solid;"></a></td>
                      <td style="line-height:12pt; height:10px;"><span class="pagetop"><b class="hnname"><a href="news">Hacker News</a></b>
                  <a href="newest">new</a> | <a href="threads?id=scudd">threads</a> | <a href="front">past</a> | <a href="newcomments">comments</a> | <a href="ask">ask</a> | <a href="show">show</a> | <a href="jobs">jobs</a> | <a href="submit">submit</a>            </span></td><td style="text-align:right;padding-right:4px;"><span class="pagetop">
                                  <a id='me' href="user?id=scudd">scudd</a>                (109) |
                    <a id='logout' href="logout?auth=bdf63b2c06b1afb6882163573bd82f6197fb4917&amp;goto=item%3Fid%3D27145911">logout</a>                          </span></td>
                  </tr></table></td></tr>
    <tr id="pagespace" title="Vulnerability allows cross-browser tracking in Chrome, Firefox, Safari, and Tor" style="height:10px"></tr><tr><td><table class="fatitem" border="0">
            <tr class='athing' id='27145911'>
          <td align="right" valign="top" class="title"><span class="rank"></span></td>      <td valign="top" class="votelinks"><center><a id='up_27145911' onclick='return vote(event, this, "up")' href='vote?id=27145911&amp;how=up&amp;auth=06ec3876e5a3b4002d33e651493ce9ce5d875055&amp;goto=item%3Fid%3D27145911'><div class='votearrow' title='upvote'></div></a></center></td><td class="title"><a href="https://fingerprintjs.com/blog/external-protocol-flooding/" class="storylink">Vulnerability allows cross-browser tracking in Chrome, Firefox, Safari, and Tor</a><span class="sitebit comhead"> (<a href="from?site=fingerprintjs.com"><span class="sitestr">fingerprintjs.com</span></a>)</span></td></tr><tr><td colspan="2"></td><td class="subtext">
            <span class="score" id="score_27145911">463 points</span> by <a href="user?id=danpinto" class="hnuser">danpinto</a> <span class="age"><a href="item?id=27145911">3 days ago</a></span> <span id="unv_27145911"></span> | <a href="flag?id=27145911&amp;auth=06ec3876e5a3b4002d33e651493ce9ce5d875055&amp;goto=item%3Fid%3D27145911">flag</a> | <a href="hide?id=27145911&amp;auth=06ec3876e5a3b4002d33e651493ce9ce5d875055&amp;goto=item%3Fid%3D27145911">hide</a> | <a href="https://hn.algolia.com/?query=Vulnerability%20allows%20cross-browser%20tracking%20in%20Chrome%2C%20Firefox%2C%20Safari%2C%20and%20Tor&type=story&dateRange=all&sort=byDate&storyText=false&prefix&page=0" class="hnpast">past</a> | <a href="fave?id=27145911&amp;auth=06ec3876e5a3b4002d33e651493ce9ce5d875055">favorite</a> | <a href="item?id=27145911">200&nbsp;comments</a>              </td></tr>
                <tr style="height:10px"></tr><tr><td colspan="2"></td><td>
              <form method="post" action="comment"><input type="hidden" name="parent" value="27145911"><input type="hidden" name="goto" value="item?id=27145911"><input type="hidden" name="hmac" value="c6c662a415c17b625636966b91a9d57adfcfdf58"><textarea name="text" rows="6" cols="60"></textarea>
                    <br><br><input type="submit" value="add comment"></form>
          </td></tr>
      </table><br><br>
      <table border="0" class='comment-tree'>
                    <tr class='athing comtr' id='27148500'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148500' onclick='return vote(event, this, "up")' href='vote?id=27148500&amp;how=up&amp;auth=134bd5a1362752c495425892ec1d50312928d7ac&amp;goto=item%3Fid%3D27145911#27148500'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=akersten" class="hnuser">akersten</a> <span class="age"><a href="item?id=27148500">2 days ago</a></span> <span id="unv_27148500"></span><span class="par"></span> <a class="togg" n="17" href="javascript:void(0)" onclick="return toggle(event, 27148500)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I&#x27;m going to close a website as soon as I get an unprompted popup that says &quot;Firefox is trying to open Slack.&quot;<p>It&#x27;s clever but somewhat obvious (in both a to-the-user-that-its-happening and a &quot;well of course it&#x27;s possible&quot; sense).<p>So it&#x27;s cute, but not practical, and I won&#x27;t lose sleep over it. I&#x27;ll probably be more inconvenienced by the mitigations that will surely result that make it that much more painful to actually launch a URL scheme, sadly.<p>I&#x27;ve actually never checked the &quot;Always open Slack for slack:&#x2F;&#x2F; links&quot; or similar checkboxes, precisely out of predicting shenanigans like this would happen eventually :)<p>I wouldn&#x27;t be too offended if browsers changed the way they handle schemes: always open a &quot;how would you like to handle  this link&quot; dialog for any protocol (even if unhandled - like how Windows shows the &quot;how would you like to open this file&quot; dialog), to disguise whether the protocol is handled or not.  Not sure I have the answer for user convenience though if someone is used to things automatically opening. That&#x27;s the &quot;inconvenience&quot; aspect of any potential mitigation, we probably have to get rid of that &quot;remember this choice&quot; checkbox (well, my point is that &quot;have to&quot; is debatable here).</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148500&amp;goto=item%3Fid%3D27145911%2327148500">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148772'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148772' onclick='return vote(event, this, "up")' href='vote?id=27148772&amp;how=up&amp;auth=92c04aaec73b988dbebd8c6109ee56fb2a18cfc7&amp;goto=item%3Fid%3D27145911#27148772'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=notRobot" class="hnuser">notRobot</a> <span class="age"><a href="item?id=27148772">2 days ago</a></span> <span id="unv_27148772"></span><span class="par"></span> <a class="togg" n="13" href="javascript:void(0)" onclick="return toggle(event, 27148772)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Note: I just tried the demo [0], and no obvious prompt showed up, instead it was a tiny window [1] on the bottom right of my screen, which only showed up for a couple seconds and is easy to miss.<p>[0]: <a href="https:&#x2F;&#x2F;schemeflood.com&#x2F;" rel="nofollow">https:&#x2F;&#x2F;schemeflood.com&#x2F;</a><p>[1]: <a href="https:&#x2F;&#x2F;imgur.com&#x2F;a&#x2F;YqbbfPt" rel="nofollow">https:&#x2F;&#x2F;imgur.com&#x2F;a&#x2F;YqbbfPt</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148772&amp;goto=item%3Fid%3D27145911%2327148772">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27152178'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27152178' onclick='return vote(event, this, "up")' href='vote?id=27152178&amp;how=up&amp;auth=70fe3bc11b6405bf7cfa2dd07dd916a13b8b77f8&amp;goto=item%3Fid%3D27145911#27152178'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=worble" class="hnuser">worble</a> <span class="age"><a href="item?id=27152178">2 days ago</a></span> <span id="unv_27152178"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27152178)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Every time I run the demo, it gives me a different result? This is just on the same browser (Firefox), it generates a different code every time and claims I have random applications installed that I don&#x27;t (the only one I have on that list is steam, which it does seems to consistently report at least). Not sure if one of my extensions is interfering with it.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27152178&amp;goto=item%3Fid%3D27145911%2327152178">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27149349'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27149349' onclick='return vote(event, this, "up")' href='vote?id=27149349&amp;how=up&amp;auth=0bdeea72de91b4d65fbfd65ed14cd1b42e42df4c&amp;goto=item%3Fid%3D27145911#27149349'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=akersten" class="hnuser">akersten</a> <span class="age"><a href="item?id=27149349">2 days ago</a></span> <span id="unv_27149349"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27149349)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Yeah, that was the popup I was referencing - although it&#x27;s much smaller for you than it was for me - maybe my low res laptop screen is a benefit there. It was noticable enough to clue me that something weird was afoot, but I&#x27;m sure it could be disguised further.<p>I think a fix could be: always show a select-program prompt even for unknown schemes (perhaps with a built-in link to the add-ons store a la Windows to find a program to open the &quot;file&quot; ;) ), never fail to a different page context than a successful launch would go to, and make the don&#x27;t-ask-again checkbox domain specific to prevent random domains doing drive-by automatic launch detection. That seems to solve it without being too disruptive to existing convenience.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149349&amp;goto=item%3Fid%3D27145911%2327149349">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149553'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27149553' onclick='return vote(event, this, "up")' href='vote?id=27149553&amp;how=up&amp;auth=460ea293fdc7f16a6c29a12a38cb33699c303355&amp;goto=item%3Fid%3D27145911#27149553'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=jonnycomputer" class="hnuser">jonnycomputer</a> <span class="age"><a href="item?id=27149553">2 days ago</a></span> <span id="unv_27149553"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149553)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I think so. I tried it on my imac, and I didn&#x27;t notice it until my second go.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149553&amp;goto=item%3Fid%3D27145911%2327149553">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27150072'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27150072' onclick='return vote(event, this, "up")' href='vote?id=27150072&amp;how=up&amp;auth=6933ecf275bc2c49e4e3ef39837b7aa829d4f393&amp;goto=item%3Fid%3D27145911#27150072'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Semaphor" class="hnuser">Semaphor</a> <span class="age"><a href="item?id=27150072">2 days ago</a></span> <span id="unv_27150072"></span><span class="par"></span> <a class="togg" n="7" href="javascript:void(0)" onclick="return toggle(event, 27150072)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I have FF set up to open all popups in new tabs. That makes it a <i>lot</i> more noticeable ;)</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150072&amp;goto=item%3Fid%3D27145911%2327150072">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27150985'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27150985' onclick='return vote(event, this, "up")' href='vote?id=27150985&amp;how=up&amp;auth=c165f04b8ed55197473d47061222c223763937aa&amp;goto=item%3Fid%3D27145911#27150985'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=simondotau" class="hnuser">simondotau</a> <span class="age"><a href="item?id=27150985">2 days ago</a></span> <span id="unv_27150985"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27150985)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I do this too. It&#x27;s also a massive usability win for those annoying websites (usually banking or government) that insist upon a full screen pop-up window to hide the navigation controls and URL bar.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150985&amp;goto=item%3Fid%3D27145911%2327150985">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27151666'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27151666' onclick='return vote(event, this, "up")' href='vote?id=27151666&amp;how=up&amp;auth=1c28a3af9704878d1b39b380adcd948819f258a8&amp;goto=item%3Fid%3D27145911#27151666'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=zuhsetaqi" class="hnuser">zuhsetaqi</a> <span class="age"><a href="item?id=27151666">2 days ago</a></span> <span id="unv_27151666"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27151666)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Which setting does this? Is it „Open links in tabs instead of windows“?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151666&amp;goto=item%3Fid%3D27145911%2327151666">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27152222'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="160"></td><td valign="top" class="votelinks">
          <center><a id='up_27152222' onclick='return vote(event, this, "up")' href='vote?id=27152222&amp;how=up&amp;auth=c88569b1632fb16f23809233b12b2e58c94aea4f&amp;goto=item%3Fid%3D27145911#27152222'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=worble" class="hnuser">worble</a> <span class="age"><a href="item?id=27152222">2 days ago</a></span> <span id="unv_27152222"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27152222)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I don&#x27;t know if it&#x27;s available in the UI, but in about:config set browser.link.open_newwindow.restriction to 0</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27152222&amp;goto=item%3Fid%3D27145911%2327152222">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27150287'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27150287' onclick='return vote(event, this, "up")' href='vote?id=27150287&amp;how=up&amp;auth=e606eecc8df8b60553d6302e3c336376c10da160&amp;goto=item%3Fid%3D27145911#27150287'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=chii" class="hnuser">chii</a> <span class="age"><a href="item?id=27150287">2 days ago</a></span> <span id="unv_27150287"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27150287)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">this should be the default behaviour imho - there should never really be a situation where a new window popup is going to be better than a tab.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150287&amp;goto=item%3Fid%3D27145911%2327150287">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151079'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="160"></td><td valign="top" class="votelinks">
          <center><a id='up_27151079' onclick='return vote(event, this, "up")' href='vote?id=27151079&amp;how=up&amp;auth=ff519edefebaf7d94465d8aa3ffbd69db7915ad9&amp;goto=item%3Fid%3D27145911#27151079'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Semaphor" class="hnuser">Semaphor</a> <span class="age"><a href="item?id=27151079">2 days ago</a></span> <span id="unv_27151079"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27151079)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I have to disagree there. It makes paypal payments really ugly, that is most certainly not what a normal user wants ;)</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151079&amp;goto=item%3Fid%3D27145911%2327151079">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27152194'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="200"></td><td valign="top" class="votelinks">
          <center><a id='up_27152194' onclick='return vote(event, this, "up")' href='vote?id=27152194&amp;how=up&amp;auth=28bd70ee2b0cb908cc0267b394d5eae4ab10bc91&amp;goto=item%3Fid%3D27145911#27152194'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=lucideer" class="hnuser">lucideer</a> <span class="age"><a href="item?id=27152194">2 days ago</a></span> <span id="unv_27152194"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27152194)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">That&#x27;s on paypal, and is something they would likely have fixed already if it were the default behaviour.<p>As for what normal users want, I would presume most of them would want to be safer on the web.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27152194&amp;goto=item%3Fid%3D27145911%2327152194">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                  <tr class='athing comtr' id='27153042'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27153042' onclick='return vote(event, this, "up")' href='vote?id=27153042&amp;how=up&amp;auth=404df3e0742bf5dea4c33304a5947dd64ace4348&amp;goto=item%3Fid%3D27145911#27153042'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=pimterry" class="hnuser">pimterry</a> <span class="age"><a href="item?id=27153042">2 days ago</a></span> <span id="unv_27153042"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27153042)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I&#x27;m using a tiling window manager and it&#x27;s <i>very</i> hard to miss: each attempt opens a new window that resizes the browser and takes up half the screen.<p>On the other hand, I guess they could automatically measure the window size in the popups and use this to detect tiling window managers, which gives them another (albeit noisy) bit for fingerprinting...</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27153042&amp;goto=item%3Fid%3D27145911%2327153042">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27153494'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27153494' onclick='return vote(event, this, "up")' href='vote?id=27153494&amp;how=up&amp;auth=9f80b9c6a551a058e26f5e6fb1e2839769bb53ff&amp;goto=item%3Fid%3D27145911#27153494'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=swiley" class="hnuser">swiley</a> <span class="age"><a href="item?id=27153494">2 days ago</a></span> <span id="unv_27153494"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27153494)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">The large number of incompatible desktop Linux configurations have underrated security benefits.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27153494&amp;goto=item%3Fid%3D27145911%2327153494">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                            <tr class='athing comtr' id='27148679'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148679' onclick='return vote(event, this, "up")' href='vote?id=27148679&amp;how=up&amp;auth=30e1ebfbfec45bbcb597afac7361b5f1142acd9f&amp;goto=item%3Fid%3D27145911#27148679'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=judge2020" class="hnuser">judge2020</a> <span class="age"><a href="item?id=27148679">2 days ago</a></span> <span id="unv_27148679"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148679)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">On Chrome MacOS Big Sur, it doesn&#x27;t require accepting the prompt, and the demo shows you can accomplish this in a small pop-under or pop-up, which a lot of inexperienced users might simply ignore.<p>Browser devs definitely still need to patch this vulnerability by making it an instant-return no-feedback prompt to open an application.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148679&amp;goto=item%3Fid%3D27145911%2327148679">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149431'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27149431' onclick='return vote(event, this, "up")' href='vote?id=27149431&amp;how=up&amp;auth=cfe0eb65bf1d26f56e87b6578ae3ab418a8140b9&amp;goto=item%3Fid%3D27145911#27149431'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=akersten" class="hnuser">akersten</a> <span class="age"><a href="item?id=27149431">2 days ago</a></span> <span id="unv_27149431"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149431)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I think my initial reaction was too harsh; after thinking through it some more I agree and think there&#x27;s an easy enough fix I posted in a sibling comment.<p>As an aside, It&#x27;s actually surprising the built-in popup blocker let <i>so many</i> popups come from just one user action - I would have thought the heuristic was 1 click = 1 allowed popup before Firefox started denying them.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149431&amp;goto=item%3Fid%3D27145911%2327149431">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27149451'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149451' onclick='return vote(event, this, "up")' href='vote?id=27149451&amp;how=up&amp;auth=2555b1c645433d4cfcbaea395fd3c3850ab82922&amp;goto=item%3Fid%3D27145911#27149451'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=laurent92" class="hnuser">laurent92</a> <span class="age"><a href="item?id=27149451">2 days ago</a></span> <span id="unv_27149451"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149451)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Remember the time of early ActiveX, when you could execute an antivirus in the browser that would scan your entire hard drive. It was exactly the same tech that happened for Windows Update executing in-browser. It feels like we’re doing the same mistake over and over.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149451&amp;goto=item%3Fid%3D27145911%2327149451">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147876'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147876' onclick='return vote(event, this, "up")' href='vote?id=27147876&amp;how=up&amp;auth=a300db20d516014b373d65fe6a7b443c28f7645b&amp;goto=item%3Fid%3D27145911#27147876'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27147876">2 days ago</a></span> <span id="unv_27147876"></span><span class="par"></span> <a class="togg" n="15" href="javascript:void(0)" onclick="return toggle(event, 27147876)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I’m the author.<p>The accuracy can be low because of:<p>- Custom browser settings or flags - The demo was designed for the default setup, but that doesn’t mean your custom setup is not vulnerable.<p>- Poorly performant hardware (including virtual machines) - Some timings are just hardcoded and were tested on the MacBook hardware.<p>- Fullscreen mode - The demo will work faster and more accurate if the browser is not in a fullscreen mode<p>- Slow internet connection<p>- Gestures during the process<p>Also, we haven’t looked into Opera yet, but we may if you ask to do it.<p>For the technical questions or bug reports consider using Github Issues</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147876&amp;goto=item%3Fid%3D27145911%2327147876">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149011'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149011' onclick='return vote(event, this, "up")' href='vote?id=27149011&amp;how=up&amp;auth=335c5c3b5a3f50069815fdece5b2c462ac565876&amp;goto=item%3Fid%3D27145911#27149011'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=hoistbypetard" class="hnuser">hoistbypetard</a> <span class="age"><a href="item?id=27149011">2 days ago</a></span> <span id="unv_27149011"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27149011)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Interesting work.<p>It didn&#x27;t work between firefox and chromium on my linux desktop, even trying the chromium branch. But my linux desktop already puts me into a pretty small bucket of users to begin with, so someone who&#x27;s doing this may not see any joy in trying to fix that.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149011&amp;goto=item%3Fid%3D27145911%2327149011">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151478'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27151478' onclick='return vote(event, this, "up")' href='vote?id=27151478&amp;how=up&amp;auth=f8f71242191fc9c43b501dc97c9b5e023086a3b1&amp;goto=item%3Fid%3D27145911#27151478'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27151478">2 days ago</a></span> <span id="unv_27151478"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151478)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Thanks.<p>Linux is tricky. Mostly because Chrome opens applications through `xdg-open`. Custom configuration on Firefox may also affect the result.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151478&amp;goto=item%3Fid%3D27145911%2327151478">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27147980'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147980' onclick='return vote(event, this, "up")' href='vote?id=27147980&amp;how=up&amp;auth=577ec35f8dc9b441c8d44879337ce88fee6ff8b6&amp;goto=item%3Fid%3D27145911#27147980'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27147980">2 days ago</a></span> <span id="unv_27147980"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147980)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I also made a special branch for Chromium (Chrome, Brave, Edge, etc.) that works much slower, but should be more accurate.<p>It still may not work for your browser with a custom configuration. Also, it is better not to make any gestures during the process.<p><a href="https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding&#x2F;tree&#x2F;feature&#x2F;chrome-long-delay" rel="nofollow">https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding&#x2F;...</a><p><a href="https:&#x2F;&#x2F;609d9f4d79c4f6000700782c--boring-visvesvaraya-dbefd4.netlify.app&#x2F;" rel="nofollow">https:&#x2F;&#x2F;609d9f4d79c4f6000700782c--boring-visvesvaraya-dbefd4...</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147980&amp;goto=item%3Fid%3D27145911%2327147980">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148098'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148098' onclick='return vote(event, this, "up")' href='vote?id=27148098&amp;how=up&amp;auth=9a9d0a5bd556aa481d24067664fedeadbe36442a&amp;goto=item%3Fid%3D27145911#27148098'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Otek" class="hnuser">Otek</a> <span class="age"><a href="item?id=27148098">2 days ago</a></span> <span id="unv_27148098"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148098)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Opera is now fully Chromium so it should be similar to others</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148098&amp;goto=item%3Fid%3D27145911%2327148098">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27153228'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27153228' onclick='return vote(event, this, "up")' href='vote?id=27153228&amp;how=up&amp;auth=07ce66c3705129791ede2d01f2a1e16d875096b6&amp;goto=item%3Fid%3D27145911#27153228'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nonameiguess" class="hnuser">nonameiguess</a> <span class="age"><a href="item?id=27153228">2 days ago</a></span> <span id="unv_27153228"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27153228)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It appears to just detect the presence of an installed scheme handler, not the application itself. This does tell you that the application was at one point installed and the uninstaller for it lies and doesn&#x27;t completely uninstall, but none of the applications it thinks I have are applications I still have (just Spotify and Skype, but still).<p>Good to know who the offenders are, Spotify and Skype. Everything else I uninstalled was actually uninstalled.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27153228&amp;goto=item%3Fid%3D27145911%2327153228">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27153310'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27153310' onclick='return vote(event, this, "up")' href='vote?id=27153310&amp;how=up&amp;auth=5e097c94b9f363a3e0bf53728ba05b21f587f67d&amp;goto=item%3Fid%3D27145911#27153310'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nonameiguess" class="hnuser">nonameiguess</a> <span class="age"><a href="item?id=27153310">2 days ago</a></span> <span id="unv_27153310"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27153310)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">For what it&#x27;s worth, I went to the Registry Editor, deleted the entries in HKEY_CLASSES_ROOT for Skype and Spotify completely, and these are still showing up as installed.<p>Makes me wonder if Windows is somehow pre-installing custom scheme handlers for these, whether you have them or not. As far as I know, Skype comes with Windows, so there is no way to test a fresh installation that never had it at all, but Spotify? Is there anyone using a completely clean fresh Windows installation that can test if this demo thinks it is installed even though it isn&#x27;t?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27153310&amp;goto=item%3Fid%3D27145911%2327153310">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27148402'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148402' onclick='return vote(event, this, "up")' href='vote?id=27148402&amp;how=up&amp;auth=2b60682e6441f9267b005f6bcf7a0333d8b688f5&amp;goto=item%3Fid%3D27145911#27148402'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=adontz" class="hnuser">adontz</a> <span class="age"><a href="item?id=27148402">2 days ago</a></span> <span id="unv_27148402"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148402)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">How come you happen to detect Xcode and Sketch on Windows?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148402&amp;goto=item%3Fid%3D27145911%2327148402">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27153175'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27153175' onclick='return vote(event, this, "up")' href='vote?id=27153175&amp;how=up&amp;auth=d3e3ee1c021418bde3460405ea10aafee9fa65d8&amp;goto=item%3Fid%3D27145911#27153175'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=h1fra" class="hnuser">h1fra</a> <span class="age"><a href="item?id=27153175">2 days ago</a></span> <span id="unv_27153175"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27153175)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Got a perfect match on Chrome vs Firefox. 
    Scary, and very easy to miss the little popup for casual user.<p>Wonder is it possible to replace the popup by an iframe?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27153175&amp;goto=item%3Fid%3D27145911%2327153175">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27154214'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27154214' onclick='return vote(event, this, "up")' href='vote?id=27154214&amp;how=up&amp;auth=6a8d13fd81a6262c4651c43d8156bda8295e0408&amp;goto=item%3Fid%3D27145911#27154214'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27154214">2 days ago</a></span> <span id="unv_27154214"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27154214)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It is possible on Tor Browser. Chrome and Firefox show a confirmation popup in the main frame.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27154214&amp;goto=item%3Fid%3D27145911%2327154214">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27149314'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149314' onclick='return vote(event, this, "up")' href='vote?id=27149314&amp;how=up&amp;auth=309cdfbb729aeddb0da89dda87248958508b9f7b&amp;goto=item%3Fid%3D27145911#27149314'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=a_t48" class="hnuser">a_t48</a> <span class="age"><a href="item?id=27149314">2 days ago</a></span> <span id="unv_27149314"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149314)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">FWIW - it worked perfectly on Firefox for Linux, but Chrome claimed I had...pretty much everything installed, so it broke horribly.<p>EDIT: the &quot;special branch&quot; also didn&#x27;t work</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149314&amp;goto=item%3Fid%3D27145911%2327149314">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27152624'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27152624' onclick='return vote(event, this, "up")' href='vote?id=27152624&amp;how=up&amp;auth=866ecc99423f499e7424688a2000dbd4939b1193&amp;goto=item%3Fid%3D27145911#27152624'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=vmception" class="hnuser">vmception</a> <span class="age"><a href="item?id=27152624">2 days ago</a></span> <span id="unv_27152624"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27152624)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">reminds me why I only use Tor on Tails</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27152624&amp;goto=item%3Fid%3D27145911%2327152624">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27149966'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149966' onclick='return vote(event, this, "up")' href='vote?id=27149966&amp;how=up&amp;auth=210ee65bfaeb0420b8e93899450bd0c986c790ad&amp;goto=item%3Fid%3D27145911#27149966'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=fomine3" class="hnuser">fomine3</a> <span class="age"><a href="item?id=27149966">2 days ago</a></span> <span id="unv_27149966"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27149966)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">The issue on Chromium bug tracker is reported by @microsoft.com. So testing on Chromium Edge would be nice.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149966&amp;goto=item%3Fid%3D27145911%2327149966">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151487'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27151487' onclick='return vote(event, this, "up")' href='vote?id=27151487&amp;how=up&amp;auth=5175d8a7390c5f5ca21c4f718697df26d0bf5352&amp;goto=item%3Fid%3D27145911#27151487'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27151487">2 days ago</a></span> <span id="unv_27151487"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151487)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Edge 90 is also affected. We tested it on Windows 10.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151487&amp;goto=item%3Fid%3D27145911%2327151487">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27150150'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27150150' onclick='return vote(event, this, "up")' href='vote?id=27150150&amp;how=up&amp;auth=efe43f881143d0a9c995fb6ee5a69a8d497ec763&amp;goto=item%3Fid%3D27145911#27150150'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=DoctorOW" class="hnuser">DoctorOW</a> <span class="age"><a href="item?id=27150150">2 days ago</a></span> <span id="unv_27150150"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27150150)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Tried it on Chromium edge and it doesn&#x27;t seem to work</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150150&amp;goto=item%3Fid%3D27145911%2327150150">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                      <tr class='athing comtr' id='27147163'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147163' onclick='return vote(event, this, "up")' href='vote?id=27147163&amp;how=up&amp;auth=8cef5631897ffe21d402b4b307b7b3407163554b&amp;goto=item%3Fid%3D27145911#27147163'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=jedberg" class="hnuser">jedberg</a> <span class="age"><a href="item?id=27147163">2 days ago</a></span> <span id="unv_27147163"></span><span class="par"></span> <a class="togg" n="7" href="javascript:void(0)" onclick="return toggle(event, 27147163)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Did it on Chrome, Firefox, and Safari and got the same code on all three.  In all three it failed to detect some apps, but the same ones failed each time.<p>When I did it in Safari it actually caused Apple Music to open.  When I did it in Chrome it popped up a small square window where I could see it doing it&#x27;s thing.<p>Firefox was the only one where it was silent.<p>But still, that&#x27;s an interesting hack.  Very clever.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147163&amp;goto=item%3Fid%3D27145911%2327147163">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148724'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148724' onclick='return vote(event, this, "up")' href='vote?id=27148724&amp;how=up&amp;auth=7111a53938f8fa0caa816d754b86b1342c03d1f9&amp;goto=item%3Fid%3D27145911#27148724'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=danudey" class="hnuser">danudey</a> <span class="age"><a href="item?id=27148724">2 days ago</a></span> <span id="unv_27148724"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148724)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I &quot;saw&quot; the little square window in Firefox on Windows 10, but only because I was paying attention. It was down in the corner, on (for some reason) my second monitor.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148724&amp;goto=item%3Fid%3D27145911%2327148724">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148787'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148787' onclick='return vote(event, this, "up")' href='vote?id=27148787&amp;how=up&amp;auth=b0cc364eea26bd63444af76a48e7f0ddfd582cc5&amp;goto=item%3Fid%3D27145911#27148787'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=notRobot" class="hnuser">notRobot</a> <span class="age"><a href="item?id=27148787">2 days ago</a></span> <span id="unv_27148787"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148787)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Yep, same experience on FF. Here&#x27;s a screenshot: <a href="https:&#x2F;&#x2F;imgur.com&#x2F;a&#x2F;YqbbfPt" rel="nofollow">https:&#x2F;&#x2F;imgur.com&#x2F;a&#x2F;YqbbfPt</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148787&amp;goto=item%3Fid%3D27145911%2327148787">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27148471'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148471' onclick='return vote(event, this, "up")' href='vote?id=27148471&amp;how=up&amp;auth=38aaece1e35f61305360101b5538c47dfcb993a6&amp;goto=item%3Fid%3D27145911#27148471'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=cdubzzz" class="hnuser">cdubzzz</a> <span class="age"><a href="item?id=27148471">2 days ago</a></span> <span id="unv_27148471"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27148471)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; When I did it in Chrome it popped up a small square window where I could see it doing it&#x27;s thing.<p>Interesting. In my case I saw the little pop up window in all three browsers. Otherwise same results though.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148471&amp;goto=item%3Fid%3D27145911%2327148471">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148704'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148704' onclick='return vote(event, this, "up")' href='vote?id=27148704&amp;how=up&amp;auth=b9a207c81d462a9ad2121a4270d2deca660e2651&amp;goto=item%3Fid%3D27145911#27148704'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=busymom0" class="hnuser">busymom0</a> <span class="age"><a href="item?id=27148704">2 days ago</a></span> <span id="unv_27148704"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148704)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c73">I just verified this on my machine and it was able to uniquely identify across Brave browser, Firefox and Epic browser (based on Chromium). I didn&#x27;t check Safari because that&#x27;s the browser I use the most and don&#x27;t want to test on that.<p>The Epic browser one was interesting. That browser comes with a built in proxy for routing connections through other countries. I use it to get around geolocked content and sometimes for a tiny sense of anonymity. But seeing this able to identify it with same identifier as Brave and Firefox was a bit more troubling. But I guess that comes with the territory of all these browsers using the same Chromium engine.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148704&amp;goto=item%3Fid%3D27145911%2327148704">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151239'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27151239' onclick='return vote(event, this, "up")' href='vote?id=27151239&amp;how=up&amp;auth=cc0cc6e6a358ac804136d2ec540aa01e4f35843e&amp;goto=item%3Fid%3D27145911#27151239'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nieve" class="hnuser">nieve</a> <span class="age"><a href="item?id=27151239">2 days ago</a></span> <span id="unv_27151239"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151239)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Firefox somewhat famously doesn&#x27;t use the same Chromium engine.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151239&amp;goto=item%3Fid%3D27145911%2327151239">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                            <tr class='athing comtr' id='27152556'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27152556' onclick='return vote(event, this, "up")' href='vote?id=27152556&amp;how=up&amp;auth=15fb3de23f04f5d7f0a2470e347e4c8ab26f3d6f&amp;goto=item%3Fid%3D27145911#27152556'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=mvanaltvorst" class="hnuser">mvanaltvorst</a> <span class="age"><a href="item?id=27152556">2 days ago</a></span> <span id="unv_27152556"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27152556)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Was silent for me on macOS Big Sur Safari as well, except for the fact that it opened Apple Music without any warning. The author might want to remove the iTunes check, not sure how much entropy it adds anyways given that it is automatically installed on all Macs.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27152556&amp;goto=item%3Fid%3D27145911%2327152556">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27148940'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148940' onclick='return vote(event, this, "up")' href='vote?id=27148940&amp;how=up&amp;auth=0886319682544586c1caf84dde1aa7333c937e33&amp;goto=item%3Fid%3D27145911#27148940'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=opheliate" class="hnuser">opheliate</a> <span class="age"><a href="item?id=27148940">2 days ago</a></span> <span id="unv_27148940"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148940)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Lots of comments about whether or not the demo works consistently between browsers, but regardless, it&#x27;s a cool attack vector, major props to the authors. Honestly surprised the Tor browser didn&#x27;t just disable protocol handlers outright beforehand, seems like a vulnerability waiting to happen when you&#x27;re that paranoid.<p>I&#x27;m a bit confused about why so many applications have bothered to create custom protocol handlers. I can see the benefit for something like Spotify, you click a link in your browser and it takes you to the song you want in the Spotify application. But is the NordVPN application really so complex that they can&#x27;t just say, &quot;hey, open Nord and click this&quot;? Just seems like an unnecessary UX decision. Unless there&#x27;s something I&#x27;m not seeing?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148940&amp;goto=item%3Fid%3D27145911%2327148940">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149615'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149615' onclick='return vote(event, this, "up")' href='vote?id=27149615&amp;how=up&amp;auth=b965885a5e56a63d4829595f86f1388fa4a5a961&amp;goto=item%3Fid%3D27145911#27149615'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dannyw" class="hnuser">dannyw</a> <span class="age"><a href="item?id=27149615">2 days ago</a></span> <span id="unv_27149615"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149615)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It&#x27;s a more integrated experience. It&#x27;s totally understandable because it&#x27;s an officially supported and endorsed way of deep linking.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149615&amp;goto=item%3Fid%3D27145911%2327149615">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27149617'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27149617' onclick='return vote(event, this, "up")' href='vote?id=27149617&amp;how=up&amp;auth=0036695719dc162c57f0b521ccbd8f9c4629af93&amp;goto=item%3Fid%3D27145911#27149617'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=chrismorgan" class="hnuser">chrismorgan</a> <span class="age"><a href="item?id=27149617">2 days ago</a></span> <span id="unv_27149617"></span><span class="par"></span> <a class="togg" n="6" href="javascript:void(0)" onclick="return toggle(event, 27149617)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">This is connected to a significant usability problem with `tel:` links: you have no way of knowing whether they’ll work, and if they <i>don’t</i> work, it could be in one of a few different ways. Maybe it’ll open a dialer app. Maybe it’ll do nothing at all. Maybe it’ll open an “unknown scheme” browser error page. Maybe it’ll prompt you to open it in an external app (I seem to have both Skype and Zoom willing to handle tel: links; neither is going to succeed). You largely can’t detect whether it has done something, might have done something, or has done nothing. Well, this article shows ways that you can detect <i>likely</i> results for some cases <i>after trying it</i>, but it’s not reliable and is depending on implementation details that are liable to change (especially since they’re a fingerprinting vector).<p>If it’s not going to work, I’d strongly prefer to not make the phone number a link—and perhaps even to present a different flow to the user (e.g. provide a form or mark an email address as the primary option). But if it is going to work, I definitely want it to be a link. It’s common to just guess from the user agent string or screen size whether it’s a mobile device, but that’s extremely flawed too—some tablets will and some won’t be able to dial, and even desktop platforms may well have some VoIP app.<p>Fingerprinting and usability are so often so significantly at odds. :-(</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149617&amp;goto=item%3Fid%3D27145911%2327149617">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27150369'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27150369' onclick='return vote(event, this, "up")' href='vote?id=27150369&amp;how=up&amp;auth=abb1fc03ad025c083417865acae96e8c0d659fa3&amp;goto=item%3Fid%3D27145911#27150369'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Scoundreller" class="hnuser">Scoundreller</a> <span class="age"><a href="item?id=27150369">2 days ago</a></span> <span id="unv_27150369"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27150369)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Same problem with loading up news.ycombinator.com in Lynx. The browser assumes I meant nntp:&#x2F;&#x2F;news.ycombinator.com ugh.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150369&amp;goto=item%3Fid%3D27145911%2327150369">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27150693'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27150693' onclick='return vote(event, this, "up")' href='vote?id=27150693&amp;how=up&amp;auth=f3e8a21eda4f5e95f2e5de24ad88d9e72184aada&amp;goto=item%3Fid%3D27145911#27150693'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=zzo38computer" class="hnuser">zzo38computer</a> <span class="age"><a href="item?id=27150693">2 days ago</a></span> <span id="unv_27150693"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27150693)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Lynx is functioning correctly (although I would prefer treating user-entered URLs as relative). However, I think that in new versions of Lynx you can turn that feature off if you want to. However, I think that they really should add a NNTP server so that you can access NNTP, too.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150693&amp;goto=item%3Fid%3D27145911%2327150693">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27150475'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27150475' onclick='return vote(event, this, "up")' href='vote?id=27150475&amp;how=up&amp;auth=752e72bb2428e2371f84d69ff3eb20af142d55ee&amp;goto=item%3Fid%3D27145911#27150475'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=danielmeskin" class="hnuser">danielmeskin</a> <span class="age"><a href="item?id=27150475">2 days ago</a></span> <span id="unv_27150475"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27150475)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Does Skype no longer do POTS calls?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150475&amp;goto=item%3Fid%3D27145911%2327150475">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27150568'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27150568' onclick='return vote(event, this, "up")' href='vote?id=27150568&amp;how=up&amp;auth=caeb87f5d11d8192878525db5be3934f0f131d1e&amp;goto=item%3Fid%3D27145911#27150568'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=chrismorgan" class="hnuser">chrismorgan</a> <span class="age"><a href="item?id=27150568">2 days ago</a></span> <span id="unv_27150568"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27150568)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">If you pay them. I say it’s not going to succeed specifically because I haven’t.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150568&amp;goto=item%3Fid%3D27145911%2327150568">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151951'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27151951' onclick='return vote(event, this, "up")' href='vote?id=27151951&amp;how=up&amp;auth=57fe336cbee16d8212347bf75e987fc1dce8eda0&amp;goto=item%3Fid%3D27145911#27151951'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=SahAssar" class="hnuser">SahAssar</a> <span class="age"><a href="item?id=27151951">2 days ago</a></span> <span id="unv_27151951"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151951)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Right, but if you&#x27;re not on a device with a built in phone connection then it&#x27;s reasonable to open skype and similar apps that can do phone calls (even if they cost money).</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151951&amp;goto=item%3Fid%3D27145911%2327151951">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                            <tr class='athing comtr' id='27148635'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148635' onclick='return vote(event, this, "up")' href='vote?id=27148635&amp;how=up&amp;auth=b6c7132354b1a3d7e0bf678c6828322057f81294&amp;goto=item%3Fid%3D27145911#27148635'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=cbsks" class="hnuser">cbsks</a> <span class="age"><a href="item?id=27148635">2 days ago</a></span> <span id="unv_27148635"></span><span class="par"></span> <a class="togg" n="9" href="javascript:void(0)" onclick="return toggle(event, 27148635)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">How do I disable this? I don&#x27;t have any need to open Skype, or any other application, from my browser. Is it a browser setting (I use Firefox) or is it an OS setting (Windows)?<p>Edit: It looks like an OS setting. In Windows the URI schemes are configured in the registry: <a href="https:&#x2F;&#x2F;stackoverflow.com&#x2F;questions&#x2F;80650&#x2F;how-do-i-register-a-custom-url-protocol-in-windows" rel="nofollow">https:&#x2F;&#x2F;stackoverflow.com&#x2F;questions&#x2F;80650&#x2F;how-do-i-register-...</a> Anyone know if there is an easy way to list all the URI schemes?<p>Edit2: After thinking about this more, I&#x27;m afraid that removing URI schemes from the registry may break those programs. I&#x27;d much rather have a browser level setting that will only open external http:&#x2F;https: resources and other URI schemes that are configured from the browser like mailto:.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148635&amp;goto=item%3Fid%3D27145911%2327148635">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27155120'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27155120' onclick='return vote(event, this, "up")' href='vote?id=27155120&amp;how=up&amp;auth=8ca7b76d22bd12364b0d701686872ed2b2e6b726&amp;goto=item%3Fid%3D27145911#27155120'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nonameiguess" class="hnuser">nonameiguess</a> <span class="age"><a href="item?id=27155120">2 days ago</a></span> <span id="unv_27155120"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27155120)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">You can remove any settings you persisted in Firefox regarding whether to open a uri scheme in an external application or not by going to your profile folder and deleting the handlers.json file. Do this when Firefox is not open. This will clear any history if you&#x27;ve ever selected &quot;always open links of this type with &lt;blah&gt;&quot; in the popup.<p>But unfortunately, this exploit is just depending on the popup to happen at all, which I don&#x27;t think you can configure from Firefox. If a uri scheme handler is registered with Windows, Firefox will ask you if you want to use it. Deleting the registered scheme handler from Windows is a matter of finding an entry in HKEY_CLASSES_ROOT in the registry with a name that matches the scheme and deleting that entry. For instance, in regedit, if you find HKEY_CLASSES_ROOT\spotify, you can delete it and no more handler for spotify:&#x2F;&#x2F;.<p>Whether or not this breaks the program probably depends on the program. If buttons and links in the application itself use this scheme, then it probably will. If they&#x27;re handled directly without delegating to the OS, then maybe not. Worst that happens is you can always just reinstall the application if it stops working.<p>I&#x27;m looking around through Firefox docs about whether it&#x27;s possible to block specific uri schemes from being handled at all but not finding anything. They do block data:&#x2F;&#x2F; and have a strict origin policy for file:&#x2F;&#x2F;, but those are already on by default and I can&#x27;t find anything related to blocking (or allowing) arbitrary uri schemes. That would be one obvious fix, though, and the researchers here did report this as a bug, so maybe an upcoming Firefox will offer this.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27155120&amp;goto=item%3Fid%3D27145911%2327155120">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27155187'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27155187' onclick='return vote(event, this, "up")' href='vote?id=27155187&amp;how=up&amp;auth=6f755ff935cd08c277e352f54cd0e4aecc22670d&amp;goto=item%3Fid%3D27145911#27155187'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nonameiguess" class="hnuser">nonameiguess</a> <span class="age"><a href="item?id=27155187">2 days ago</a></span> <span id="unv_27155187"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27155187)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Should add the obvious ultimate way to prevent fingerprinting of this type is to just run Firefox in its own VM or container with a totally clean OS you otherwise don&#x27;t touch. You could choose to share the Downloads folder between guest and host so you can still save files, but it then wouldn&#x27;t be able to see what you do and don&#x27;t have installed on your real host system.<p>Of course, if you do anything to allow hardware acceleration in your browser so you&#x27;re not streaming media like it&#x27;s 1999, it&#x27;ll still be able to fingerprint you based on the hardware, but at least it won&#x27;t see what applications you have.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27155187&amp;goto=item%3Fid%3D27145911%2327155187">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27148797'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148797' onclick='return vote(event, this, "up")' href='vote?id=27148797&amp;how=up&amp;auth=4287056df1f09658fe27542f8eba39f945e6dd27&amp;goto=item%3Fid%3D27145911#27148797'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=notRobot" class="hnuser">notRobot</a> <span class="age"><a href="item?id=27148797">2 days ago</a></span> <span id="unv_27148797"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27148797)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Here&#x27;s one way to disable it on Windows: <a href="https:&#x2F;&#x2F;www.thewindowsclub.com&#x2F;how-to-prevent-launching-apps-associated-with-file-or-uri-scheme" rel="nofollow">https:&#x2F;&#x2F;www.thewindowsclub.com&#x2F;how-to-prevent-launching-apps...</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148797&amp;goto=item%3Fid%3D27145911%2327148797">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148904'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148904' onclick='return vote(event, this, "up")' href='vote?id=27148904&amp;how=up&amp;auth=accaf89ae40418deed7578b74101dbe0171d179b&amp;goto=item%3Fid%3D27145911#27148904'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=cbsks" class="hnuser">cbsks</a> <span class="age"><a href="item?id=27148904">2 days ago</a></span> <span id="unv_27148904"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148904)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">The Local Group Policy setting in the link only affects Windows Store apps:<p>&quot;This policy setting lets you control whether Windows Store apps can open URIs using the default desktop app for a URI scheme. Because desktop apps run at a higher integrity level than Windows Store apps, there is a risk that a URI scheme launched by a Windows Store app might compromise the system by launching a desktop app.&quot;<p>I haven&#x27;t tried the registry setting, maybe that will also block normal desktop applications? Edit: it looks like the BlockProtocolElevation setting also only affects Windows Store apps.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148904&amp;goto=item%3Fid%3D27145911%2327148904">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149111'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27149111' onclick='return vote(event, this, "up")' href='vote?id=27149111&amp;how=up&amp;auth=d342b6a0245dea8e09ff5ab58fdfd1bcab69132b&amp;goto=item%3Fid%3D27145911#27149111'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=notRobot" class="hnuser">notRobot</a> <span class="age"><a href="item?id=27149111">2 days ago</a></span> <span id="unv_27149111"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149111)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">That&#x27;s unfortunate :(</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149111&amp;goto=item%3Fid%3D27145911%2327149111">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                            <tr class='athing comtr' id='27149396'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149396' onclick='return vote(event, this, "up")' href='vote?id=27149396&amp;how=up&amp;auth=7bf8153888611759a863f7d50b9dd89ab46da2eb&amp;goto=item%3Fid%3D27145911#27149396'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=anonymfus" class="hnuser">anonymfus</a> <span class="age"><a href="item?id=27149396">2 days ago</a></span> <span id="unv_27149396"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27149396)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00"><i>&gt; Anyone know if there is an easy way to list all the URI schemes?</i><p><i>Settings</i> → <i>Apps</i> → <i>Default apps</i> → <i>Choose default apps by protocol</i><p>Also there is <i>Settings</i> → <i>Apps</i> → <i>Apps for websites</i>, where you can control rerouting of http&#x2F;https links to applications.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149396&amp;goto=item%3Fid%3D27145911%2327149396">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149503'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27149503' onclick='return vote(event, this, "up")' href='vote?id=27149503&amp;how=up&amp;auth=7d45969a0cb0ac762b1b8a025db24706b0244798&amp;goto=item%3Fid%3D27145911#27149503'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Psychlist" class="hnuser">Psychlist</a> <span class="age"><a href="item?id=27149503">2 days ago</a></span> <span id="unv_27149503"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149503)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Windows Settings lets me choose an app, but not choose no app&#x2F;remove an app. So that&#x27;s not entirely useful (the obvious &quot;just use notepad for everything&quot; is also an obvious tell)<p>I&#x27;m not wildly keep on manually editing the registry, at least without someone else doing it first and reporting that it didn&#x27;t break their computer :)</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149503&amp;goto=item%3Fid%3D27145911%2327149503">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27149113'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149113' onclick='return vote(event, this, "up")' href='vote?id=27149113&amp;how=up&amp;auth=3f211f706dedbc9fc7e250a3b576611594595b36&amp;goto=item%3Fid%3D27145911#27149113'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=busymom0" class="hnuser">busymom0</a> <span class="age"><a href="item?id=27149113">2 days ago</a></span> <span id="unv_27149113"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149113)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">For MacOS, I was able to fix this by editing the info.plist inside each application which was detected. This lets me still keep the app but no longer get detected.<p>WARNING: Do it at your own risk. I am fairly certain when I restart my computer, the Spotify app will no longer work as deleting the entry from the info.plist file most likely changes the signature of the app binary and it will no longer be valid.<p>Simply uninstalling the app won&#x27;t be enough. Rebuild LaunchServices is required to get rid of the registered URL scheme.<p>The info.plist for Spotify for example is located at:<p>&#x2F;Applications&#x2F;Spotify.app&#x2F;Contents&#x2F;Info.plist<p>You can either do it through terminal or navigate to &#x2F;Applications in finder, then right click the app and use &quot;Show Package Content&quot; option &gt; Contents &gt; Info.plist.<p>Open the Info.plist in Xcode, look for CFBundleURLSchemes:<p>&lt;array&gt;
     &lt;dict&gt;
      &lt;key&gt;CFBundleTypeRole&lt;&#x2F;key&gt;
      &lt;string&gt;Viewer&lt;&#x2F;string&gt;
      &lt;key&gt;CFBundleURLIconFile&lt;&#x2F;key&gt;
      &lt;string&gt;&lt;&#x2F;string&gt;
      &lt;key&gt;CFBundleURLName&lt;&#x2F;key&gt;
      &lt;string&gt;Spotify Media&lt;&#x2F;string&gt;
      &lt;key&gt;CFBundleURLSchemes&lt;&#x2F;key&gt;
      &lt;array&gt;
       &lt;string&gt;spotify&lt;&#x2F;string&gt;
      &lt;&#x2F;array&gt;
     &lt;&#x2F;dict&gt;
    &lt;&#x2F;array&gt;<p>I removed this array. Save the file.<p>NOTE that if you previously had the app installed in a different directory, you might have to do it there too.<p>Once done, you will have to run this command to &quot;Rebuild LaunchServices&quot; as explained on this Stack Overflow post.<p><a href="https:&#x2F;&#x2F;stackoverflow.com&#x2F;questions&#x2F;10156939&#x2F;mac-show-delete-custom-url-schemes" rel="nofollow">https:&#x2F;&#x2F;stackoverflow.com&#x2F;questions&#x2F;10156939&#x2F;mac-show-delete...</a><p>&#x2F;System&#x2F;Library&#x2F;Frameworks&#x2F;CoreServices.framework&#x2F;Frameworks&#x2F;LaunchServices.framework&#x2F;Support&#x2F;lsregister -kill -r -domain local -domain system -domain user<p>Without the above command, the URL scheme wasn&#x27;t getting unregistered and the site was still picking it up.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149113&amp;goto=item%3Fid%3D27145911%2327149113">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147144'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147144' onclick='return vote(event, this, "up")' href='vote?id=27147144&amp;how=up&amp;auth=fc64f8a927cf723ba37301b8e10d1f890d4a6f1e&amp;goto=item%3Fid%3D27145911#27147144'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=jraph" class="hnuser">jraph</a> <span class="age"><a href="item?id=27147144">2 days ago</a></span> <span id="unv_27147144"></span><span class="par"></span> <a class="togg" n="6" href="javascript:void(0)" onclick="return toggle(event, 27147144)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">On Linux:<p>- in Firefox, it detected Epic Games Telegram Discord Battle.net Xcode NordVPN Sketch Teamviewer Microsoft Word WhatsApp Postman Adobe Messenger Figma Hotspot Shield ExpressVPN Notion iTunes, none of which I have installed. It didn&#x27;t detect VSCode though I have VSCodium.<p>- On Chromium, it warned it would not work well on Chrome on Linux. It incorrectly detected all the apps. It seems that the browser would try to open the links with xdg-open.<p>Clever hack anyway!</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147144&amp;goto=item%3Fid%3D27145911%2327147144">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148426'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148426' onclick='return vote(event, this, "up")' href='vote?id=27148426&amp;how=up&amp;auth=43f9c8d0e3214c858755996edd785c9c2f253159&amp;goto=item%3Fid%3D27145911#27148426'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=DistressedDrone" class="hnuser">DistressedDrone</a> <span class="age"><a href="item?id=27148426">2 days ago</a></span> <span id="unv_27148426"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148426)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Using Firefox on Linux, it detected all the apps (very few of which I have) except Skype (correct, I don&#x27;t have it).<p>Security through obscurity does it again!</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148426&amp;goto=item%3Fid%3D27145911%2327148426">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27147829'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147829' onclick='return vote(event, this, "up")' href='vote?id=27147829&amp;how=up&amp;auth=4706757dc472b0979c5f91b86ea8a4f3c911e4db&amp;goto=item%3Fid%3D27145911#27147829'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=valve1" class="hnuser">valve1</a> <span class="age"><a href="item?id=27147829">2 days ago</a></span> <span id="unv_27147829"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147829)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Thanks for testing it on Linux. We only tested it on these browser + OS combinations: <a href="https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding#target-browsers" rel="nofollow">https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding#...</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147829&amp;goto=item%3Fid%3D27145911%2327147829">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151144'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27151144' onclick='return vote(event, this, "up")' href='vote?id=27151144&amp;how=up&amp;auth=20b765ff9b5391a2df444019b4b8cc27e388e4a8&amp;goto=item%3Fid%3D27145911#27151144'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=jraph" class="hnuser">jraph</a> <span class="age"><a href="item?id=27151144">2 days ago</a></span> <span id="unv_27151144"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151144)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">if you need more info: Firefox 88.0.1 (64 bits), Chromium 90.0.4430.93, on openSUSE Tumbleweed</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151144&amp;goto=item%3Fid%3D27145911%2327151144">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27149273'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149273' onclick='return vote(event, this, "up")' href='vote?id=27149273&amp;how=up&amp;auth=d3df2d160243ee7bbc06e7da79b28c78ce851ad3&amp;goto=item%3Fid%3D27145911#27149273'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=psanford" class="hnuser">psanford</a> <span class="age"><a href="item?id=27149273">2 days ago</a></span> <span id="unv_27149273"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149273)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Also Chrome on Linux. For me it says it detected 11 apps installed, but I don&#x27;t have any of those apps installed. Strange.<p>Edit: With firefox it is able to correctly detect 3 installed desktop applications.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149273&amp;goto=item%3Fid%3D27145911%2327149273">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27151474'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27151474' onclick='return vote(event, this, "up")' href='vote?id=27151474&amp;how=up&amp;auth=ded890f973a903526b6c98ba24f6f772d810c8d8&amp;goto=item%3Fid%3D27145911#27151474'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=jeltz" class="hnuser">jeltz</a> <span class="age"><a href="item?id=27151474">2 days ago</a></span> <span id="unv_27151474"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151474)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Yup, it was broken for me too on Linux unless I somehow have managed to install both Xcode and MS Word. :)</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151474&amp;goto=item%3Fid%3D27145911%2327151474">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147050'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147050' onclick='return vote(event, this, "up")' href='vote?id=27147050&amp;how=up&amp;auth=7ee1bbe247c54a38ec2036e77f53f4fe33d496d7&amp;goto=item%3Fid%3D27145911#27147050'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nimbius" class="hnuser">nimbius</a> <span class="age"><a href="item?id=27147050">2 days ago</a></span> <span id="unv_27147050"></span><span class="par"></span> <a class="togg" n="10" href="javascript:void(0)" onclick="return toggle(event, 27147050)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt;By opening a popup window with a custom URL scheme and checking if its document is available from JavaScript code, you can detect if the application is installed on the device.<p>in FF, unless im mistaken this assumes the user clicks anything except cancel on the popup.  bug for reference and comment. 
    <a href="https:&#x2F;&#x2F;bugzilla.mozilla.org&#x2F;show_bug.cgi?id=1711084" rel="nofollow">https:&#x2F;&#x2F;bugzilla.mozilla.org&#x2F;show_bug.cgi?id=1711084</a><p>further from the github:<p>&gt; the basic concept is the same. It works by asking the browser to show a confirmation dialog in a popup window. Then the JavaScript code can detect if a popup has just been opened and detect the presence of an application based on that.<p>so...we seem to be relying on the honor system with the user?  Can anyone clarify?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147050&amp;goto=item%3Fid%3D27145911%2327147050">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147214'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147214' onclick='return vote(event, this, "up")' href='vote?id=27147214&amp;how=up&amp;auth=5a7e48a18a214f6e0116dc9003eaf96247e19b1e&amp;goto=item%3Fid%3D27145911#27147214'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27147214">2 days ago</a></span> <span id="unv_27147214"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147214)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Hi, nimbius.<p>I’m the article author, can you please clarify your question?<p>The demo will not work without a popup window in Chrome, Firefox and Safari. The “Get My Identifier” button is needed in order to have a single user gesture to open an additional window.<p>However the Tor Browser demo works silently without any additional window.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147214&amp;goto=item%3Fid%3D27145911%2327147214">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148820'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148820' onclick='return vote(event, this, "up")' href='vote?id=27148820&amp;how=up&amp;auth=c9fcf996dcefa88518065ea478256fa15efbfe8a&amp;goto=item%3Fid%3D27145911#27148820'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=busymom0" class="hnuser">busymom0</a> <span class="age"><a href="item?id=27148820">2 days ago</a></span> <span id="unv_27148820"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148820)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c5a">On Firefox, I didn&#x27;t get any popup window. I did get it on Brave browser (Chromium based).</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148820&amp;goto=item%3Fid%3D27145911%2327148820">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27148025'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148025' onclick='return vote(event, this, "up")' href='vote?id=27148025&amp;how=up&amp;auth=572f26059fc72a65772527bc10ae520fa45f8579&amp;goto=item%3Fid%3D27145911#27148025'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=chmod775" class="hnuser">chmod775</a> <span class="age"><a href="item?id=27148025">2 days ago</a></span> <span id="unv_27148025"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148025)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; in FF, unless im mistaken this assumes the user clicks anything except cancel on the popup. bug for reference and comment.<p>I&#x27;m on Firefox and didn&#x27;t have to click anything. It correctly detected I have Steam installed.<p>The flashing popup window was quite obvious though.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148025&amp;goto=item%3Fid%3D27145911%2327148025">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27147727'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147727' onclick='return vote(event, this, "up")' href='vote?id=27147727&amp;how=up&amp;auth=0214de97eb0d8a6f22a31e8e1c769ea0579ca224&amp;goto=item%3Fid%3D27145911#27147727'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=tacticalmook" class="hnuser">tacticalmook</a> <span class="age"><a href="item?id=27147727">2 days ago</a></span> <span id="unv_27147727"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147727)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; It works by asking the browser to show a confirmation dialog in a popup window. Then the JavaScript code can detect if a popup has just been opened and detect the presence of an application based on that.<p>&gt; ...<p>&gt; Tor Browser has confirmation dialogs disabled entirely as a privacy feature, which, ironically, exposed a more damaging vulnerability for this particular exploit. Nothing is shown while the exploit runs in the background, contrasting with other browsers that show pop-ups during the process.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147727&amp;goto=item%3Fid%3D27145911%2327147727">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27147193'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147193' onclick='return vote(event, this, "up")' href='vote?id=27147193&amp;how=up&amp;auth=a9bc7d5754a65412ccce34092e8c9288a108b42c&amp;goto=item%3Fid%3D27145911#27147193'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dathinab" class="hnuser">dathinab</a> <span class="age"><a href="item?id=27147193">2 days ago</a></span> <span id="unv_27147193"></span><span class="par"></span> <a class="togg" n="5" href="javascript:void(0)" onclick="return toggle(event, 27147193)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Basically browsers have the &quot;I open a popup to ask&quot; or &quot;the user has no schema handler for that schema so I don&#x27;t need to ask&quot; or the &quot;User already confirmed it always should open the link with given application&quot; behaviour and they can detect it &quot;somehow &quot;?<p>But I still have to look closer into it.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147193&amp;goto=item%3Fid%3D27145911%2327147193">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147363'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147363' onclick='return vote(event, this, "up")' href='vote?id=27147363&amp;how=up&amp;auth=55c8428ef0dc0d0be9e79db21d5ecda8c6f060d8&amp;goto=item%3Fid%3D27145911#27147363'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=valve1" class="hnuser">valve1</a> <span class="age"><a href="item?id=27147363">2 days ago</a></span> <span id="unv_27147363"></span><span class="par"></span> <a class="togg" n="4" href="javascript:void(0)" onclick="return toggle(event, 27147363)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Browsers open pop-ups to ask &quot;Can I run that application?&quot; but only if that application is installed. If that application is not installed, the browser will ignore the custom URL.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147363&amp;goto=item%3Fid%3D27145911%2327147363">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148672'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27148672' onclick='return vote(event, this, "up")' href='vote?id=27148672&amp;how=up&amp;auth=eda203d1fd11326afa42fbcdbc36d1dc0ec42fb8&amp;goto=item%3Fid%3D27145911#27148672'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=jszymborski" class="hnuser">jszymborski</a> <span class="age"><a href="item?id=27148672">2 days ago</a></span> <span id="unv_27148672"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27148672)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It looks like a mitigation might be that in the event you do not have the application installed, to return a &quot;denied&quot; status and send a prompt to the user like &quot;Unknown application protocol&quot;.<p>Something like that could still would be susceptible to a timing attack though.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148672&amp;goto=item%3Fid%3D27145911%2327148672">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27153406'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="160"></td><td valign="top" class="votelinks">
          <center><a id='up_27153406' onclick='return vote(event, this, "up")' href='vote?id=27153406&amp;how=up&amp;auth=1e03f2354fac5b978c78249b75e7dfd5d32513a3&amp;goto=item%3Fid%3D27145911#27153406'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Haemm0r" class="hnuser">Haemm0r</a> <span class="age"><a href="item?id=27153406">2 days ago</a></span> <span id="unv_27153406"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27153406)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">always show the popup, but populate it &quot;later&quot; could work too.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27153406&amp;goto=item%3Fid%3D27145911%2327153406">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27161108'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="200"></td><td valign="top" class="votelinks">
          <center><a id='up_27161108' onclick='return vote(event, this, "up")' href='vote?id=27161108&amp;how=up&amp;auth=2901c430f670a6e9a8c8ce1fe35ba01ef7511536&amp;goto=item%3Fid%3D27145911#27161108'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dathinab" class="hnuser">dathinab</a> <span class="age"><a href="item?id=27161108">1 day ago</a></span> <span id="unv_27161108"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27161108)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Yes I believe the proper fix would be to always behave as if a popup is showing, independent of weather or not it actually shows.<p>Through it&#x27;s maybe slightly more complex as you might need to behave as if the user clicked cancel in a way where a attacker can not easily differentiate it from an actual user clicking cancel.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27161108&amp;goto=item%3Fid%3D27145911%2327161108">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                                        <tr class='athing comtr' id='27148840'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148840' onclick='return vote(event, this, "up")' href='vote?id=27148840&amp;how=up&amp;auth=cdaba605e0b6fb1d2b4baa7b3793d1ceaff090a8&amp;goto=item%3Fid%3D27145911#27148840'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=tempestn" class="hnuser">tempestn</a> <span class="age"><a href="item?id=27148840">2 days ago</a></span> <span id="unv_27148840"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148840)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I tried it in Firefox and Tor, and got the same identifier for both, but in both cases it said, &quot;This is your identifier. It is unique among [####] tests so far.&quot;<p>But.. it wasn&#x27;t unique for the second browser I tried.  And they&#x27;d be coming from different IPs, so it wouldn&#x27;t have any way to know both were coming from the same person, aside from the fingerprinting itself...</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148840&amp;goto=item%3Fid%3D27145911%2327148840">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27146879'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27146879' onclick='return vote(event, this, "up")' href='vote?id=27146879&amp;how=up&amp;auth=6cc0d4a47e444bb42bbf9931d81a90cd461e27cd&amp;goto=item%3Fid%3D27145911#27146879'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=bronzeage" class="hnuser">bronzeage</a> <span class="age"><a href="item?id=27146879">2 days ago</a></span> <span id="unv_27146879"></span><span class="par"></span> <a class="togg" n="6" href="javascript:void(0)" onclick="return toggle(event, 27146879)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Looking at their product, I wonder how many of these kind of vulnerabilities are still open and exploited by them. Wouldn&#x27;t make much sense for them to burn such a useful vulnerability which is required for their product unless they had something better.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27146879&amp;goto=item%3Fid%3D27145911%2327146879">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147310'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147310' onclick='return vote(event, this, "up")' href='vote?id=27147310&amp;how=up&amp;auth=c4f72bf70ec699c93d9ae2f057975da1c92f3b38&amp;goto=item%3Fid%3D27145911#27147310'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dathinab" class="hnuser">dathinab</a> <span class="age"><a href="item?id=27147310">2 days ago</a></span> <span id="unv_27147310"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27147310)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">You can get a lot of entropy just by fingerprinting things send over HTTP headers and things freely accessible by JS.<p>E.g. user agent, screen dimensions, language, web GL, audio api, etc.<p>Generally wrt. fingerprinting chrome is worse then Firefox  as Firefox actively worked to reduce fingerprint-ability if possible, while chrome seems to not care much. Because of this ironically I have a less unique fingerprint on a customized Firefox browser then a &quot;stock&quot; Chrome browser even through much less people use Firefox...<p>The reason (I think) why they make this public is because this can be used for more then &quot;just&quot; fingerprinting. I.e. this can be used by cyber attacks to find a potential attack vector to then pull of either a direct attack or some social engineering attack.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147310&amp;goto=item%3Fid%3D27145911%2327147310">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148605'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148605' onclick='return vote(event, this, "up")' href='vote?id=27148605&amp;how=up&amp;auth=92662c9bc81f36f9789239ff5f5d98c4c24e7a3c&amp;goto=item%3Fid%3D27145911#27148605'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=the_duke" class="hnuser">the_duke</a> <span class="age"><a href="item?id=27148605">2 days ago</a></span> <span id="unv_27148605"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148605)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Firefox also has a lot of settings that mitigate various finger printing techniques. There are some good sample configs on Github. [1]<p>Ironically, many of the settings can make you more unique because they disable a lot of functionality.<p>[1] <a href="https:&#x2F;&#x2F;github.com&#x2F;pyllyukko&#x2F;user.js" rel="nofollow">https:&#x2F;&#x2F;github.com&#x2F;pyllyukko&#x2F;user.js</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148605&amp;goto=item%3Fid%3D27145911%2327148605">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27150113'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27150113' onclick='return vote(event, this, "up")' href='vote?id=27150113&amp;how=up&amp;auth=ad6d319541bcccdbee3f2fa3a3c6ef8963897192&amp;goto=item%3Fid%3D27145911#27150113'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=KirillPanov" class="hnuser">KirillPanov</a> <span class="age"><a href="item?id=27150113">2 days ago</a></span> <span id="unv_27150113"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27150113)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">This is why the torbrowser&#x2F;firefox &quot;try to make everybody look the same&quot; approach is doomed.<p>Adding white noise is the only solution: &quot;try to make your fingerprint on each website for each brower-restart look as different as possible (a) from your fingerprint on every other website and (b) from your fingerprint on the same website on a previous browser-restart&quot;.<p>That&#x27;s the best you can do anyways without rejecting first-party cookies.<p>Brave does this, and it is the right way.  I just wish Firefox would wake up and clue in to this.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150113&amp;goto=item%3Fid%3D27145911%2327150113">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                            <tr class='athing comtr' id='27146930'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27146930' onclick='return vote(event, this, "up")' href='vote?id=27146930&amp;how=up&amp;auth=1c0c9767802dd2909dbb7c1de4956d0b980bd4ce&amp;goto=item%3Fid%3D27145911#27146930'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=harikb" class="hnuser">harikb</a> <span class="age"><a href="item?id=27146930">2 days ago</a></span> <span id="unv_27146930"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27146930)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; DISCLAIMER: FingerprintJS does not use this vulnerability in our products and does not provide third-party tracking services</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27146930&amp;goto=item%3Fid%3D27145911%2327146930">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27147237'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147237' onclick='return vote(event, this, "up")' href='vote?id=27147237&amp;how=up&amp;auth=60c5f5624848b87c5bf6854dcbc3dcca1299eb47&amp;goto=item%3Fid%3D27145911#27147237'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=grishka" class="hnuser">grishka</a> <span class="age"><a href="item?id=27147237">2 days ago</a></span> <span id="unv_27147237"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147237)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Interesting to see how their product is open source, too: <a href="https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;fingerprintjs&#x2F;" rel="nofollow">https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;fingerprintjs&#x2F;</a><p>It&#x27;s as if they <i>want</i> browser developers to look at the code and break it as much as possible.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147237&amp;goto=item%3Fid%3D27145911%2327147237">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147327'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147327' onclick='return vote(event, this, "up")' href='vote?id=27147327&amp;how=up&amp;auth=aa33ac9d5cc65b36ebe5077b387f670976e790d4&amp;goto=item%3Fid%3D27145911#27147327'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=skykooler" class="hnuser">skykooler</a> <span class="age"><a href="item?id=27147327">2 days ago</a></span> <span id="unv_27147327"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147327)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Interestingly, custom URL handlers seem to stick around even after the app associated with them has been uninstalled. For example, this detected Messenger&#x27;s URL handler although I uninstalled it a year ago.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147327&amp;goto=item%3Fid%3D27145911%2327147327">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148225'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148225' onclick='return vote(event, this, "up")' href='vote?id=27148225&amp;how=up&amp;auth=357f55e55551f5f74c1a018b6ad619e4ee059e10&amp;goto=item%3Fid%3D27145911#27148225'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=rkagerer" class="hnuser">rkagerer</a> <span class="age"><a href="item?id=27148225">2 days ago</a></span> <span id="unv_27148225"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148225)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Not the least bit surprised.  I use Total Uninstall and almost every app leaves bits behind.<p>I&#x27;ve complained to many vendors and sent technical details of missed registry keys, files, etc. 
    Sometimes they even fix it. 
    But on the whole, Uninstall on Windows is a bit of a myth.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148225&amp;goto=item%3Fid%3D27145911%2327148225">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27151358'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27151358' onclick='return vote(event, this, "up")' href='vote?id=27151358&amp;how=up&amp;auth=3d0632f378448a624f4969c47785f514bc24df60&amp;goto=item%3Fid%3D27145911#27151358'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=matheusmoreira" class="hnuser">matheusmoreira</a> <span class="age"><a href="item?id=27151358">2 days ago</a></span> <span id="unv_27151358"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27151358)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; Add a script on a website that will test each application from your list.<p>This means exploitation requires Javascript, right? Tor browser users should have it disabled at all times.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151358&amp;goto=item%3Fid%3D27145911%2327151358">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151427'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27151427' onclick='return vote(event, this, "up")' href='vote?id=27151427&amp;how=up&amp;auth=7aef3c6f2af309e590b477c77d9c125f5d1b6c99&amp;goto=item%3Fid%3D27145911#27151427'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=vaylian" class="hnuser">vaylian</a> <span class="age"><a href="item?id=27151427">2 days ago</a></span> <span id="unv_27151427"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151427)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Not in the default configuration</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151427&amp;goto=item%3Fid%3D27145911%2327151427">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27149074'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27149074' onclick='return vote(event, this, "up")' href='vote?id=27149074&amp;how=up&amp;auth=928c8dab15c11c494f60d6f7736e808fbee574ed&amp;goto=item%3Fid%3D27145911#27149074'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=theo-born" class="hnuser"><font color="#3c963c">theo-born</font></a> <span class="age"><a href="item?id=27149074">2 days ago</a></span> <span id="unv_27149074"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27149074)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I thought this was known for a while?<p>But speaking of, does the website know if you do have MetaMask installed right away (without prompting you for anything)? Because that would be a real concern if it did.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149074&amp;goto=item%3Fid%3D27145911%2327149074">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149625'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149625' onclick='return vote(event, this, "up")' href='vote?id=27149625&amp;how=up&amp;auth=dce1be3ae5c14196b96f94bc72fa0e80538b2880&amp;goto=item%3Fid%3D27145911#27149625'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dannyw" class="hnuser">dannyw</a> <span class="age"><a href="item?id=27149625">2 days ago</a></span> <span id="unv_27149625"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149625)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Yeah it does. Browsing the Web with MetaMask is like walking around with a bag of $10,000 cash openly exposed in a crowded public street of a society with no police officers or law enforcement.<p>And with a blindfold on your eyes.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149625&amp;goto=item%3Fid%3D27145911%2327149625">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27149114'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149114' onclick='return vote(event, this, "up")' href='vote?id=27149114&amp;how=up&amp;auth=0e8de42cfc71730ecb6215fb7e433ab2c35cde26&amp;goto=item%3Fid%3D27145911#27149114'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=doopy1" class="hnuser">doopy1</a> <span class="age"><a href="item?id=27149114">2 days ago</a></span> <span id="unv_27149114"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149114)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I think metamask injects web3 into every context on the page so it&#x27;s pretty easy to check for that.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149114&amp;goto=item%3Fid%3D27145911%2327149114">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147424'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147424' onclick='return vote(event, this, "up")' href='vote?id=27147424&amp;how=up&amp;auth=3cbe0082cc26a20b4161ffb7346fe919b8a9595b&amp;goto=item%3Fid%3D27145911#27147424'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=burk96" class="hnuser">burk96</a> <span class="age"><a href="item?id=27147424">2 days ago</a></span> <span id="unv_27147424"></span><span class="par"></span> <a class="togg" n="6" href="javascript:void(0)" onclick="return toggle(event, 27147424)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Worked perfectly on Firefox 88.0.1 on Windows. Great to know despite my efforts to balance privacy and anonymity, there is another metric that I&#x27;m unique in. Fingerprinting is just insidious.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147424&amp;goto=item%3Fid%3D27145911%2327147424">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147495'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147495' onclick='return vote(event, this, "up")' href='vote?id=27147495&amp;how=up&amp;auth=4dd233bfa3a433f64cba60773e163aec52e8a5de&amp;goto=item%3Fid%3D27145911#27147495'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=sneak" class="hnuser">sneak</a> <span class="age"><a href="item?id=27147495">2 days ago</a></span> <span id="unv_27147495"></span><span class="par"></span> <a class="togg" n="5" href="javascript:void(0)" onclick="return toggle(event, 27147495)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Browsing in a VM is really one of the only safe ways to go on the modern web for privacy. So many sites break without JS, and having it enabled is an accident waiting to happen.<p>When you need privacy, always browse in a VM or a Tails boot.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147495&amp;goto=item%3Fid%3D27145911%2327147495">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147656'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147656' onclick='return vote(event, this, "up")' href='vote?id=27147656&amp;how=up&amp;auth=3533bdb98837484ea0a1cee7a91e1a2de9638c0b&amp;goto=item%3Fid%3D27145911#27147656'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=chithanh" class="hnuser">chithanh</a> <span class="age"><a href="item?id=27147656">2 days ago</a></span> <span id="unv_27147656"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147656)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Even in a VM you have to carefully ensure that memory deduplication is disabled, and&#x2F;or some form of mitigation against Rowhammer is in place. Else you will be vulnerable to Flip Feng Shui cross-VM attacks.<p><a href="https:&#x2F;&#x2F;fahrplan.events.ccc.de&#x2F;congress&#x2F;2016&#x2F;Fahrplan&#x2F;events&#x2F;8022.html" rel="nofollow">https:&#x2F;&#x2F;fahrplan.events.ccc.de&#x2F;congress&#x2F;2016&#x2F;Fahrplan&#x2F;events...</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147656&amp;goto=item%3Fid%3D27145911%2327147656">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27147725'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147725' onclick='return vote(event, this, "up")' href='vote?id=27147725&amp;how=up&amp;auth=87dae0fc44ba3b0b40cb6fe88cf95116dfdeb8af&amp;goto=item%3Fid%3D27145911#27147725'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Dah00n" class="hnuser">Dah00n</a> <span class="age"><a href="item?id=27147725">2 days ago</a></span> <span id="unv_27147725"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27147725)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">This won&#x27;t work against fingerprinting unless you change the underlying hardware and &#x2F; or external IP too when stating a new VM. If you don&#x27;t have a unique external IP per VM you might as well not bother. It is like trying to hide from the police by changing clothes and cutting your hair but stil hold the same huge sign with your name and address in your hands.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147725&amp;goto=item%3Fid%3D27145911%2327147725">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147866'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27147866' onclick='return vote(event, this, "up")' href='vote?id=27147866&amp;how=up&amp;auth=c1d66a16d1c212298d9b4e44204b9e182e25a762&amp;goto=item%3Fid%3D27145911#27147866'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=sneak" class="hnuser">sneak</a> <span class="age"><a href="item?id=27147866">2 days ago</a></span> <span id="unv_27147866"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147866)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">The use of Tor or a public VPN (i.e. many hundreds of unrelated users sharing a single public IP) is implicit.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147866&amp;goto=item%3Fid%3D27145911%2327147866">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27160163'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="160"></td><td valign="top" class="votelinks">
          <center><a id='up_27160163' onclick='return vote(event, this, "up")' href='vote?id=27160163&amp;how=up&amp;auth=3340c6a0cdaa95ec741e7dea10c46d2b6255c96e&amp;goto=item%3Fid%3D27145911#27160163'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Dah00n" class="hnuser">Dah00n</a> <span class="age"><a href="item?id=27160163">1 day ago</a></span> <span id="unv_27160163"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27160163)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Since this is about fingerprinting and not hiding your identity I&#x27;m not sure this will help. If you use a public VPN you are removing some data points from the fingerprint but adding a huge new one. After all fingerprinting is about blending in and being like the average user. Adding a few privacy extensions and a VPN and you are much easier to recognise.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27160163&amp;goto=item%3Fid%3D27145911%2327160163">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                                  <tr class='athing comtr' id='27147110'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147110' onclick='return vote(event, this, "up")' href='vote?id=27147110&amp;how=up&amp;auth=9b95b62e5abd786ff6b16c67359b318335304cfd&amp;goto=item%3Fid%3D27145911#27147110'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=matsemann" class="hnuser">matsemann</a> <span class="age"><a href="item?id=27147110">2 days ago</a></span> <span id="unv_27147110"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147110)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Interesting concept. Most fingerprinting I&#x27;ve seen so far has for instance used the GPU to detect small differences in rendering, but also based on browser. First cross-browser I&#x27;ve seen, barring the obvious stuff like IP or so.<p>Hope this won&#x27;t be a post where everyone that didn&#x27;t get the same identifier have to proclaim it, though. We get it, it&#x27;s not perfect. FWIW I got same in Edge &amp; Fx and it claimed it was a unique combo (different ID in Chrome, though).</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147110&amp;goto=item%3Fid%3D27145911%2327147110">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27151909'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27151909' onclick='return vote(event, this, "up")' href='vote?id=27151909&amp;how=up&amp;auth=c79ace12eaa06b3feb1e3594b425a96b3c9b4d75&amp;goto=item%3Fid%3D27145911#27151909'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=saagarjha" class="hnuser">saagarjha</a> <span class="age"><a href="item?id=27151909">2 days ago</a></span> <span id="unv_27151909"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151909)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Interesting to see that browsers are still vulnerable to this; I know iOS had a very similar problem a while back (apps would check for the existence of hundreds of other applications by checking whether they could open those URL schemes) and Apple clamped down on it quickly by restricting the number of queries that could be made.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151909&amp;goto=item%3Fid%3D27145911%2327151909">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148932'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148932' onclick='return vote(event, this, "up")' href='vote?id=27148932&amp;how=up&amp;auth=dc05b9841f52dea1036ba0fe5bf970c2bae79b50&amp;goto=item%3Fid%3D27145911#27148932'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=DoomHotel" class="hnuser">DoomHotel</a> <span class="age"><a href="item?id=27148932">2 days ago</a></span> <span id="unv_27148932"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148932)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Wow, it didn&#x27;t work <i>at all</i> on my desktop. It thinks I have 23 apps from its list installed, on both Firefox and Chrome. Pretty funny seeing that on a Linux box running CentOS 7. Even better, it detects a different app on each as the only one missing: on Firefox it says I don&#x27;t have Skype installed, while on Chrome it says I don&#x27;t have Hotspot Shield installed.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148932&amp;goto=item%3Fid%3D27145911%2327148932">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149645'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149645' onclick='return vote(event, this, "up")' href='vote?id=27149645&amp;how=up&amp;auth=a9d3918c2a49d5560e76ade3b62c7a7a31674050&amp;goto=item%3Fid%3D27145911#27149645'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dannyw" class="hnuser">dannyw</a> <span class="age"><a href="item?id=27149645">2 days ago</a></span> <span id="unv_27149645"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149645)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">That&#x27;s because on Linux xdg-open handles everything. What&#x27;s missing is probably a timing issue.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149645&amp;goto=item%3Fid%3D27145911%2327149645">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27150852'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27150852' onclick='return vote(event, this, "up")' href='vote?id=27150852&amp;how=up&amp;auth=8f18adea183ee5cc7c93d9308fa8a5ca2887050e&amp;goto=item%3Fid%3D27145911#27150852'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=zzo38computer" class="hnuser">zzo38computer</a> <span class="age"><a href="item?id=27150852">2 days ago</a></span> <span id="unv_27150852"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27150852)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I would fix perhaps by the new web browser doing:<p>- Document scripts are restricted, and can be customized and spoofed (or fully disabled) by the user.<p>- Whether or not a link can be opened, and whether or not it is asked, depends on user settings. If it is configured to ask, it does so for both known and unknown URI schemes.<p>- Known and unknown schemes are both considered different origins; they do not redirect to about:blank (unless it is a scheme which is handled by rendering a document, which happens to redirect to about:blank, but it does not normally do this).<p>- Scripts cannot detect such prompts, and only one can be displayed at a time. One key combination can be used to prevent further prompts; even if a way is found, only one will work anyways.<p>And many other improvements, because existing web browsers are bad in a lot of ways.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150852&amp;goto=item%3Fid%3D27145911%2327150852">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148315'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148315' onclick='return vote(event, this, "up")' href='vote?id=27148315&amp;how=up&amp;auth=c95b228d97399269fb03b6d974f9567b125064af&amp;goto=item%3Fid%3D27145911#27148315'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=adontz" class="hnuser">adontz</a> <span class="age"><a href="item?id=27148315">2 days ago</a></span> <span id="unv_27148315"></span><span class="par"></span> <a class="togg" n="5" href="javascript:void(0)" onclick="return toggle(event, 27148315)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Results differ wildly between browsers and even between runs within the same browser. It detects application I do not have installed and does not detect applications I do have installed. For instance it detects iTunes, XCode and Sketch, but they are Mac-only application and I am on Windows.<p>Honestly, I believe it does not work at all.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148315&amp;goto=item%3Fid%3D27145911%2327148315">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148394'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148394' onclick='return vote(event, this, "up")' href='vote?id=27148394&amp;how=up&amp;auth=50c3c60ed39cf2440b5a833ccc273a1ec32dc143&amp;goto=item%3Fid%3D27145911#27148394'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=valve1" class="hnuser">valve1</a> <span class="age"><a href="item?id=27148394">2 days ago</a></span> <span id="unv_27148394"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148394)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Thanks for testing it on Windows. We mostly tested it on MacOS Big Sur because all devs on the team have that OS.
    With Windows different timings might be needed, we&#x27;ll check into it tomorrow.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148394&amp;goto=item%3Fid%3D27145911%2327148394">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148439'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148439' onclick='return vote(event, this, "up")' href='vote?id=27148439&amp;how=up&amp;auth=349412c14bff50f9c36e975d6acef190bb497bf4&amp;goto=item%3Fid%3D27145911#27148439'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=mcpherrinm" class="hnuser">mcpherrinm</a> <span class="age"><a href="item?id=27148439">2 days ago</a></span> <span id="unv_27148439"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148439)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">On my Windows&#x2F;Firefox computer, it appears to have correctly identified which 6 of the applications I have installed.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148439&amp;goto=item%3Fid%3D27145911%2327148439">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27148554'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148554' onclick='return vote(event, this, "up")' href='vote?id=27148554&amp;how=up&amp;auth=8ccf8c702599d84af75c47e07a37842c718a6528&amp;goto=item%3Fid%3D27145911#27148554'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=inetknght" class="hnuser">inetknght</a> <span class="age"><a href="item?id=27148554">2 days ago</a></span> <span id="unv_27148554"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148554)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; <i>iTunes</i><p>&gt; <i>they are Mac-only application</i><p>I remember installing and using iTunes on Windows 7.<p>It might be that Apple doesn&#x27;t distribute a modern version of iTunes. But it&#x27;s certainly not true in the past.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148554&amp;goto=item%3Fid%3D27145911%2327148554">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27152459'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27152459' onclick='return vote(event, this, "up")' href='vote?id=27152459&amp;how=up&amp;auth=64054a468db99e47a7eb24259fd0295e84b0c3e3&amp;goto=item%3Fid%3D27145911#27152459'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=adontz" class="hnuser">adontz</a> <span class="age"><a href="item?id=27152459">2 days ago</a></span> <span id="unv_27152459"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27152459)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I do not have any iTunes installed anyway.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27152459&amp;goto=item%3Fid%3D27145911%2327152459">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                      <tr class='athing comtr' id='27148037'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148037' onclick='return vote(event, this, "up")' href='vote?id=27148037&amp;how=up&amp;auth=5908f5ff416d3082a3550baf0bf5983127fd2d59&amp;goto=item%3Fid%3D27145911#27148037'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=asddubs" class="hnuser">asddubs</a> <span class="age"><a href="item?id=27148037">2 days ago</a></span> <span id="unv_27148037"></span><span class="par"></span> <a class="togg" n="4" href="javascript:void(0)" onclick="return toggle(event, 27148037)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">On my firefox (linux) it seems to think I have everything installed for some reason. Worked on tor browser though</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148037&amp;goto=item%3Fid%3D27145911%2327148037">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149647'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27149647' onclick='return vote(event, this, "up")' href='vote?id=27149647&amp;how=up&amp;auth=81521e5a5236cd89dd1d4ce51006ea5022f30d47&amp;goto=item%3Fid%3D27145911#27149647'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dannyw" class="hnuser">dannyw</a> <span class="age"><a href="item?id=27149647">2 days ago</a></span> <span id="unv_27149647"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27149647)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Xdg-open</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149647&amp;goto=item%3Fid%3D27145911%2327149647">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27150244'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27150244' onclick='return vote(event, this, "up")' href='vote?id=27150244&amp;how=up&amp;auth=4d0c3f0b5e7a9f23124643c9fe1912493b5d12ee&amp;goto=item%3Fid%3D27145911#27150244'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=KirillPanov" class="hnuser">KirillPanov</a> <span class="age"><a href="item?id=27150244">2 days ago</a></span> <span id="unv_27150244"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27150244)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Am I the only one who utterly loathes this tool?<p>Seriously, it maintains (among other things, yes) the mapping from filename extensions to the path of the binary that should be used to open them.<p>It&#x27;s a map : extension -&gt; path.<p>WhyTF does MIME have to get dragged into this?  Why can&#x27;t I just say &quot;*.foo is opened with &#x2F;usr&#x2F;bin&#x2F;foobalize&quot;?  Why must I suffer the agony of trawling the interwebs to find out that blartz.foo is actually a z-content-flavor&#x2F;foobalized_v3? 
     (Yes, I understand why <i>browsers</i> need to start the lookup using a MIME type.  I&#x27;m talking about everything else -- the galaxy of things that don&#x27;t use HTTP).<p>And even once I&#x27;ve found the Magic MIME type, xdg-open still does whatever it wants, and there appears to be no way to troubleshoot it when it&#x27;s being invoked by another application.  Setting XDG_UTILS_DEBUG_LEVEL=999 simply prints out a list of which files its reading (I can get that from strace, thanks), with no step-by-step rundown of its decision process:<p><pre><code>   $ XDG_UTILS_DEBUG_LEVEL=999 xdg-open ftp:&#x2F;&#x2F;foo.com
       Selected DE generic
       Checking &#x2F;home&#x2F;user&#x2F;.config&#x2F;mimeapps.list
       Checking &#x2F;home&#x2F;user&#x2F;.local&#x2F;share&#x2F;applications&#x2F;defaults.list and &#x2F;home&#x2F;user&#x2F;.local&#x2F;share&#x2F;applications&#x2F;mimeinfo.cache
       Checking &#x2F;home&#x2F;user&#x2F;.local&#x2F;share&#x2F;applications&#x2F;defaults.list and &#x2F;home&#x2F;user&#x2F;.local&#x2F;share&#x2F;applications&#x2F;mimeinfo.cache
       Checking &#x2F;usr&#x2F;local&#x2F;share&#x2F;&#x2F;applications&#x2F;defaults.list and &#x2F;usr&#x2F;local&#x2F;share&#x2F;&#x2F;applications&#x2F;mimeinfo.cache
       Checking &#x2F;usr&#x2F;local&#x2F;share&#x2F;&#x2F;applications&#x2F;defaults.list and &#x2F;usr&#x2F;local&#x2F;share&#x2F;&#x2F;applications&#x2F;mimeinfo.cache 
       Checking &#x2F;usr&#x2F;share&#x2F;&#x2F;applications&#x2F;defaults.list and &#x2F;usr&#x2F;share&#x2F;&#x2F;applications&#x2F;mimeinfo.cache
       Checking &#x2F;usr&#x2F;share&#x2F;&#x2F;applications&#x2F;defaults.list and &#x2F;usr&#x2F;share&#x2F;&#x2F;applications&#x2F;mimeinfo.cache
    </code></pre>
    Okay, y&#x27;all can downvote me now, ranty time is over.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150244&amp;goto=item%3Fid%3D27145911%2327150244">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27153199'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27153199' onclick='return vote(event, this, "up")' href='vote?id=27153199&amp;how=up&amp;auth=158cb00310415beba6f544d40baedf1fdda618ae&amp;goto=item%3Fid%3D27145911#27153199'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=q3k" class="hnuser">q3k</a> <span class="age"><a href="item?id=27153199">2 days ago</a></span> <span id="unv_27153199"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27153199)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">No, xdg-utils are absolutely terrible. They&#x27;re a mess of untested, undebuggable, underdocumented and extremely user unfriendly shell scripts that need to die and be replaced with something that actually had some serious design thought put into it.<p>I&#x27;ve once had xdg-open be absolutely broken on my machine, scanning all of my $HOME because of a file with a space character in it [1]. Any attempt to use xdg-open would pin a CPU core for 100% while bash&#x2F;find recursively traversed millions of files because of missing quote characters in a shell script. Truly the pinnacle of software engineering.<p>I wouldn&#x27;t be surprised if serious security bugs lurked somewhere in it, exploitable by web pages attempting to open maliciously crafted protocol URLs.<p>[1] - <a href="https:&#x2F;&#x2F;github.com&#x2F;freedesktop&#x2F;xdg-utils&#x2F;commit&#x2F;9816ebb3e6fd9f23e993b8b7fcbd56f92d9c9197" rel="nofollow">https:&#x2F;&#x2F;github.com&#x2F;freedesktop&#x2F;xdg-utils&#x2F;commit&#x2F;9816ebb3e6fd...</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27153199&amp;goto=item%3Fid%3D27145911%2327153199">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                            <tr class='athing comtr' id='27147678'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147678' onclick='return vote(event, this, "up")' href='vote?id=27147678&amp;how=up&amp;auth=2edc0242788a5f32c6cde7373e4b9ac9dbc5cb80&amp;goto=item%3Fid%3D27145911#27147678'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=gruez" class="hnuser">gruez</a> <span class="age"><a href="item?id=27147678">2 days ago</a></span> <span id="unv_27147678"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147678)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">This seems wildly inaccurate for me. On firefox with resistfingerprinting it says I have 23 of the 24 applications installed (I don&#x27;t, that&#x27;s more incorrect than correct), and on tor browser it says 0 applications installed (also incorrect, I have a few installed).</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147678&amp;goto=item%3Fid%3D27145911%2327147678">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147867'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147867' onclick='return vote(event, this, "up")' href='vote?id=27147867&amp;how=up&amp;auth=3d2dd9a97d6acfe372245449d83e24de08ddda29&amp;goto=item%3Fid%3D27145911#27147867'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=viseztrance" class="hnuser">viseztrance</a> <span class="age"><a href="item?id=27147867">2 days ago</a></span> <span id="unv_27147867"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147867)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Strange. I have resist fingerprinting as well (running on fedora), and it correctly detected all 5 apps I had installed from the list.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147867&amp;goto=item%3Fid%3D27145911%2327147867">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27150432'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27150432' onclick='return vote(event, this, "up")' href='vote?id=27150432&amp;how=up&amp;auth=fc9704ece2ad54a5fa2c39dc021ec79ec8249cf2&amp;goto=item%3Fid%3D27145911#27150432'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=graderjs" class="hnuser">graderjs</a> <span class="age"><a href="item?id=27150432">2 days ago</a></span> <span id="unv_27150432"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27150432)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Super clever to make this exploit work on Chrome by opening a PDF file prior to launching the custom app scheme, in order to activate the Chrome PDF viewer extension, which resets the global flag requiring a user gesture before any custom scheme launch. It didn&#x27;t track me across Firefox, and Chrome on Windows 10, but it was still cool.<p>Weird that when I tried running it in chrome headless[0], it opened the popup window, and tried the first scheme, but then stopped, and hung.<p>[0]: <a href="https:&#x2F;&#x2F;comebrowsewithme.com:8002&#x2F;" rel="nofollow">https:&#x2F;&#x2F;comebrowsewithme.com:8002&#x2F;</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150432&amp;goto=item%3Fid%3D27145911%2327150432">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27146954'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27146954' onclick='return vote(event, this, "up")' href='vote?id=27146954&amp;how=up&amp;auth=18e23d298d2942cbe1f9424586e6ab4d9c9cf13e&amp;goto=item%3Fid%3D27145911#27146954'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=buggeryorkshire" class="hnuser">buggeryorkshire</a> <span class="age"><a href="item?id=27146954">2 days ago</a></span> <span id="unv_27146954"></span><span class="par"></span> <a class="togg" n="9" href="javascript:void(0)" onclick="return toggle(event, 27146954)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I&#x27;ve no idea whether it works, but they misidentified many apps I don&#x27;t have installed (Postman, Express VPN, Notion, Figma, Hotspot Shield)<p>It does do the popup for VSCode asking if I want to open links there, which I do have installed.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27146954&amp;goto=item%3Fid%3D27145911%2327146954">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147227'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147227' onclick='return vote(event, this, "up")' href='vote?id=27147227&amp;how=up&amp;auth=aa18d470274d2a32a0f28b0ac5e182a4c82ba2c1&amp;goto=item%3Fid%3D27145911#27147227'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dathinab" class="hnuser">dathinab</a> <span class="age"><a href="item?id=27147227">2 days ago</a></span> <span id="unv_27147227"></span><span class="par"></span> <a class="togg" n="4" href="javascript:void(0)" onclick="return toggle(event, 27147227)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I guess (and just that), that this can happen if there are overlaps in the scheme handlers.<p>I.e. there are some schemas which lets say XCode handles but which also some other program handles.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147227&amp;goto=item%3Fid%3D27145911%2327147227">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147253'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147253' onclick='return vote(event, this, "up")' href='vote?id=27147253&amp;how=up&amp;auth=81663971a27a6bfc497662b0e0002e8cf177311b&amp;goto=item%3Fid%3D27145911#27147253'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=buggeryorkshire" class="hnuser">buggeryorkshire</a> <span class="age"><a href="item?id=27147253">2 days ago</a></span> <span id="unv_27147253"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27147253)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Yeah makes sense if it&#x27;s the schema handlers. I&#x27;d just not be as assertive if I was them that something was installed if there was overlap.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147253&amp;goto=item%3Fid%3D27145911%2327147253">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147636'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27147636' onclick='return vote(event, this, "up")' href='vote?id=27147636&amp;how=up&amp;auth=60e0d2d61fe81fdaed371b4c3d0d1e6e5568b881&amp;goto=item%3Fid%3D27145911#27147636'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dathinab" class="hnuser">dathinab</a> <span class="age"><a href="item?id=27147636">2 days ago</a></span> <span id="unv_27147636"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147636)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It also doesn&#x27;t work at all under Chromium for Linux no idea why but the result is complete garbage.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147636&amp;goto=item%3Fid%3D27145911%2327147636">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147978'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="160"></td><td valign="top" class="votelinks">
          <center><a id='up_27147978' onclick='return vote(event, this, "up")' href='vote?id=27147978&amp;how=up&amp;auth=3f6c4b0f73d283311e38543aefed6ce151eedfe5&amp;goto=item%3Fid%3D27145911#27147978'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=valve1" class="hnuser">valve1</a> <span class="age"><a href="item?id=27147978">2 days ago</a></span> <span id="unv_27147978"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147978)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">yeah, chrome&#x2F;chromium on linux not tested at all, mostly because nobody on the team is using linux. We tested it on MacOS Big Sur and a bit of Windows.
    Full table of what was tested here: <a href="https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding#target-browsers" rel="nofollow">https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding#...</a> dathinab</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147978&amp;goto=item%3Fid%3D27145911%2327147978">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                  <tr class='athing comtr' id='27147166'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147166' onclick='return vote(event, this, "up")' href='vote?id=27147166&amp;how=up&amp;auth=26019125904a973c196f5c243eabb2a360ff8dd0&amp;goto=item%3Fid%3D27145911#27147166'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=yjftsjthsd-h" class="hnuser">yjftsjthsd-h</a> <span class="age"><a href="item?id=27147166">2 days ago</a></span> <span id="unv_27147166"></span><span class="par"></span> <a class="togg" n="4" href="javascript:void(0)" onclick="return toggle(event, 27147166)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Yeah, it gave me quite a list of programs, including xcode and itunes, which is <i>fascinating</i> on a Linux box... they list 20 programs they think I have installed, of which I actually have 2. I&#x27;m not sure <i>why</i> it would be so inaccurate, but I feel better...</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147166&amp;goto=item%3Fid%3D27145911%2327147166">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147299'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147299' onclick='return vote(event, this, "up")' href='vote?id=27147299&amp;how=up&amp;auth=4f761e69c174ecd7f98d9ccda5630c380732309a&amp;goto=item%3Fid%3D27145911#27147299'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nolok" class="hnuser">nolok</a> <span class="age"><a href="item?id=27147299">2 days ago</a></span> <span id="unv_27147299"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27147299)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; I&#x27;m not sure why it would be so inaccurate, but I feel better...<p>I don&#x27;t think you understood the core of the issue: it&#x27;s not about identifying which applications you have installed, it&#x27;s about always getting the same result for the same user. If all your browsers serve the same results, you are trackable, no matter if those results are good or not.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147299&amp;goto=item%3Fid%3D27145911%2327147299">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147702'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27147702' onclick='return vote(event, this, "up")' href='vote?id=27147702&amp;how=up&amp;auth=87a5ad8325f23c8054943a690864fd6d5dde0101&amp;goto=item%3Fid%3D27145911#27147702'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=filmfact" class="hnuser">filmfact</a> <span class="age"><a href="item?id=27147702">2 days ago</a></span> <span id="unv_27147702"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147702)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I think the implication is that this is far fewer bits of entropy than the authors indicate. Four bits (in isolation), are not a meaningful identifer.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147702&amp;goto=item%3Fid%3D27145911%2327147702">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148175'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="160"></td><td valign="top" class="votelinks">
          <center><a id='up_27148175' onclick='return vote(event, this, "up")' href='vote?id=27148175&amp;how=up&amp;auth=5375f3c497f466902b7c6472d88c23887973bca2&amp;goto=item%3Fid%3D27145911#27148175'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nolok" class="hnuser">nolok</a> <span class="age"><a href="item?id=27148175">2 days ago</a></span> <span id="unv_27148175"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148175)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It&#x27;s not four, the fact that the others applications are reliably detected as not present are additional bits.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148175&amp;goto=item%3Fid%3D27145911%2327148175">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                                  <tr class='athing comtr' id='27147497'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147497' onclick='return vote(event, this, "up")' href='vote?id=27147497&amp;how=up&amp;auth=dc6cbdf177b875cbde084764c301b66848d0bab9&amp;goto=item%3Fid%3D27145911#27147497'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=butz" class="hnuser">butz</a> <span class="age"><a href="item?id=27147497">2 days ago</a></span> <span id="unv_27147497"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147497)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">At least 9 of those programs could be &quot;installed to desktop&quot; on supported Chromium based browsers. That not only lowers your fingerprint in this particular vulnerability, but also saves quite a bit of disk space.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147497&amp;goto=item%3Fid%3D27145911%2327147497">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27152455'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27152455' onclick='return vote(event, this, "up")' href='vote?id=27152455&amp;how=up&amp;auth=67f7eb86cbc9c5a8652875114753a488a4fa412b&amp;goto=item%3Fid%3D27145911#27152455'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=finder02" class="hnuser"><font color="#3c963c">finder02</font></a> <span class="age"><a href="item?id=27152455">2 days ago</a></span> <span id="unv_27152455"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27152455)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I started a little project to mitigate risk like this - run firefox from unprivileged podman container (it will work with docker too). <a href="https:&#x2F;&#x2F;github.com&#x2F;grzegorzk&#x2F;ff_in_podman" rel="nofollow">https:&#x2F;&#x2F;github.com&#x2F;grzegorzk&#x2F;ff_in_podman</a><p>I always get the same ID on the demo because 0 apps are detected :) and this makes the browser unique because not many people run browser on system with 0 apps installed.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27152455&amp;goto=item%3Fid%3D27145911%2327152455">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27146917'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27146917' onclick='return vote(event, this, "up")' href='vote?id=27146917&amp;how=up&amp;auth=d27edb4597d7195d2198d300b9539002a22699ff&amp;goto=item%3Fid%3D27145911#27146917'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=harikb" class="hnuser">harikb</a> <span class="age"><a href="item?id=27146917">2 days ago</a></span> <span id="unv_27146917"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27146917)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; Profiling based on installed apps<p>&gt; most browsers have safety mechanisms in place designed to prevent such exploits. Weaknesses in these safety mechanisms are what makes this vulnerability possible.<p>&gt; By specification, extensions need to be able to open custom URLs, such as mailto: links, without confirmation dialogs. The scheme flood protection conflicts with extension policies so there is a loophole that resets this flag every time any extension is triggered<p>If true, this sounds worse revelation than the exploit itself. Disabling a flag temporarily sounds bad, regardless of whether a vulnerability exists.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27146917&amp;goto=item%3Fid%3D27145911%2327146917">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148428'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148428' onclick='return vote(event, this, "up")' href='vote?id=27148428&amp;how=up&amp;auth=e4da9d277454ebf6e977d45256092e8bae00e6f2&amp;goto=item%3Fid%3D27145911#27148428'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nanis" class="hnuser">nanis</a> <span class="age"><a href="item?id=27148428">2 days ago</a></span> <span id="unv_27148428"></span><span class="par"></span> <a class="togg" n="7" href="javascript:void(0)" onclick="return toggle(event, 27148428)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Curious:<p>&gt; We have generated your identifier based on 1 applications you have installed.<p><pre><code>    Skype
    </code></pre>
    Then it told me I am ninety-something percent unique...<p>I find that odd because pretty much every Windows machine has Skype.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148428&amp;goto=item%3Fid%3D27145911%2327148428">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148457'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148457' onclick='return vote(event, this, "up")' href='vote?id=27148457&amp;how=up&amp;auth=5d0b11e9c2304db91a1e55bbb8f9736760873a6b&amp;goto=item%3Fid%3D27145911#27148457'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=SavannahJS" class="hnuser">SavannahJS</a> <span class="age"><a href="item?id=27148457">2 days ago</a></span> <span id="unv_27148457"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148457)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">(I work at FingerprintJS)<p>You are likely relatively unique because you only have Skype installed, whereas a lot of visitors will have more applications out of the list. Someone who has no applications on the list installed may be even more unique, for example.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148457&amp;goto=item%3Fid%3D27145911%2327148457">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148578'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148578' onclick='return vote(event, this, "up")' href='vote?id=27148578&amp;how=up&amp;auth=ffe1bacb17437be23d5deb8c2c9543561dce9eb2&amp;goto=item%3Fid%3D27145911#27148578'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=inetknght" class="hnuser">inetknght</a> <span class="age"><a href="item?id=27148578">2 days ago</a></span> <span id="unv_27148578"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148578)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; <i>Someone who has no applications on the list installed may be even more unique, for example.</i><p>I ran it in a VM with Firefox and nothing else installed. It correctly detected nothing and stated:<p>&gt; <i>This is your identifier. It was seen 273 times among 3830 tests so far.</i>
    &gt; <i>That means it is 92.87% unique.</i></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148578&amp;goto=item%3Fid%3D27145911%2327148578">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27148551'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148551' onclick='return vote(event, this, "up")' href='vote?id=27148551&amp;how=up&amp;auth=2dd427f25530fa45bac6994a6fc0f3af137fd24f&amp;goto=item%3Fid%3D27145911#27148551'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=tinus_hn" class="hnuser">tinus_hn</a> <span class="age"><a href="item?id=27148551">2 days ago</a></span> <span id="unv_27148551"></span><span class="par"></span> <a class="togg" n="4" href="javascript:void(0)" onclick="return toggle(event, 27148551)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">You also have none of the other tested applications; I presume most of them have Word.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148551&amp;goto=item%3Fid%3D27145911%2327148551">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148921'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148921' onclick='return vote(event, this, "up")' href='vote?id=27148921&amp;how=up&amp;auth=67dc44a35d25c1c803151a21e157ff5d71e41b1b&amp;goto=item%3Fid%3D27145911#27148921'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nanis" class="hnuser">nanis</a> <span class="age"><a href="item?id=27148921">2 days ago</a></span> <span id="unv_27148921"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27148921)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; You also have none of the other tested applications; I presume most of them have Word.<p>What makes you assume I do not have Office installed? Instead of, say, considering the possibility that the fingerprinting may not be that good.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148921&amp;goto=item%3Fid%3D27145911%2327148921">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151798'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27151798' onclick='return vote(event, this, "up")' href='vote?id=27151798&amp;how=up&amp;auth=da46ec0131040d0c04b084b00f08bd397237a6b6&amp;goto=item%3Fid%3D27145911#27151798'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=tinus_hn" class="hnuser">tinus_hn</a> <span class="age"><a href="item?id=27151798">2 days ago</a></span> <span id="unv_27151798"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27151798)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">The main clue is that you are not noting that it is misdetecting anything, just that you think what it is that it is detecting is not very special.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151798&amp;goto=item%3Fid%3D27145911%2327151798">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27161193'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="160"></td><td valign="top" class="votelinks">
          <center><a id='up_27161193' onclick='return vote(event, this, "up")' href='vote?id=27161193&amp;how=up&amp;auth=f5478e9e83bc03c5dde3ef886918fa43ed0a28b6&amp;goto=item%3Fid%3D27145911#27161193'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nanis" class="hnuser">nanis</a> <span class="age"><a href="item?id=27161193">1 day ago</a></span> <span id="unv_27161193"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27161193)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I noted that it only detected Skype and nothing else and that app is installed on all Windows machines these days.<p>Tried again, same deal:<p>&gt; That means it is 96.35% unique</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27161193&amp;goto=item%3Fid%3D27145911%2327161193">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                                  <tr class='athing comtr' id='27154491'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27154491' onclick='return vote(event, this, "up")' href='vote?id=27154491&amp;how=up&amp;auth=0da34d718e16a4e7fddf7f473d3adf0d174c6a4f&amp;goto=item%3Fid%3D27145911#27154491'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=toomanybeersies" class="hnuser">toomanybeersies</a> <span class="age"><a href="item?id=27154491">2 days ago</a></span> <span id="unv_27154491"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27154491)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; In a quick search of the web, we couldn’t find any website actively exploiting it but we still felt the need to report it as soon as possible.<p>I&#x27;ve seen popup-based exploits on less-legal websites (e.g. torrents, keygens, illegal streaming of live sports and&#x2F;or movies) a few times over the years. I&#x27;m unsure if they&#x27;re executing this specific exploit though.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27154491&amp;goto=item%3Fid%3D27145911%2327154491">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147610'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147610' onclick='return vote(event, this, "up")' href='vote?id=27147610&amp;how=up&amp;auth=f581750bfbabd0056974b46512a5ab6c0b38784c&amp;goto=item%3Fid%3D27145911#27147610'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=seumars" class="hnuser">seumars</a> <span class="age"><a href="item?id=27147610">2 days ago</a></span> <span id="unv_27147610"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147610)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Fingerprinting and profiling in general just makes me not want to use the internet sometimes. I stopped using gmail at the very least. Maybe I should start using a VPN.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147610&amp;goto=item%3Fid%3D27145911%2327147610">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147602'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147602' onclick='return vote(event, this, "up")' href='vote?id=27147602&amp;how=up&amp;auth=d80848099612fe7e58c9128b16c99ca0bc5d0544&amp;goto=item%3Fid%3D27145911#27147602'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=difosfor" class="hnuser">difosfor</a> <span class="age"><a href="item?id=27147602">2 days ago</a></span> <span id="unv_27147602"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27147602)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">How unique are these ids really? I imagine certain apps will be very commonly installed as well as certain groups of apps? So it&#x27;s not 32bits of information. Still more information to add to the finger printing pile..<p>I wish we could find a way to deal with this risk that&#x27;s not simply disabling all kinds of functionality. Browser APIs seem to be suffering more and more by limitations to prevent finger printing.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147602&amp;goto=item%3Fid%3D27145911%2327147602">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148106'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148106' onclick='return vote(event, this, "up")' href='vote?id=27148106&amp;how=up&amp;auth=edba93b2701721d9ea6eb449a822c26cc3c6b5f1&amp;goto=item%3Fid%3D27145911#27148106'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Sebb767" class="hnuser">Sebb767</a> <span class="age"><a href="item?id=27148106">2 days ago</a></span> <span id="unv_27148106"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148106)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; How unique are these ids really? I imagine certain apps will be very commonly installed as well as certain groups of apps?<p>Probably worse than you think. Zoom, Skype and Slack will be very common on work computers, while game launchers like steam and epic will work quite well on gaming pcs. You can differentiate further by checking the mixing of those groups and their relative music client (Spotify, ITunes...). Of course it won&#x27;t be full 32 bits, but given the amount of quite common programs with url handler, it will probably deliver quite good results.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148106&amp;goto=item%3Fid%3D27145911%2327148106">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27151598'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27151598' onclick='return vote(event, this, "up")' href='vote?id=27151598&amp;how=up&amp;auth=909bcda79e80a744bfeca47a37d5121c5939a51d&amp;goto=item%3Fid%3D27145911#27151598'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27151598">2 days ago</a></span> <span id="unv_27151598"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151598)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">We will make a detailed report with some statistics, after the vulnerability is fixed</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151598&amp;goto=item%3Fid%3D27145911%2327151598">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147773'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147773' onclick='return vote(event, this, "up")' href='vote?id=27147773&amp;how=up&amp;auth=6ea33f8b73b436bef0b1b77b3bbdaeb128b3f307&amp;goto=item%3Fid%3D27145911#27147773'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=elmo2you" class="hnuser">elmo2you</a> <span class="age"><a href="item?id=27147773">2 days ago</a></span> <span id="unv_27147773"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147773)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Aside from profiling, can these custom URL handlers also be used as an attack vector on other installed applications?<p>That is, assuming any of those happens to be installed and have a (input sanitation related) vulnerability.<p>Maybe I&#x27;m just seeing ghosts here. But the idea of a web site pushing malicious links to whatever software may also be installed on the same machine, isn&#x27;t a very comforting thought.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147773&amp;goto=item%3Fid%3D27145911%2327147773">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151621'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27151621' onclick='return vote(event, this, "up")' href='vote?id=27151621&amp;how=up&amp;auth=94dec1190ba0d41bc330baefc1749bfdec91fdc0&amp;goto=item%3Fid%3D27145911#27151621'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27151621">2 days ago</a></span> <span id="unv_27151621"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151621)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">This is possible in theory.<p>For example, Safari opens the Apple Music without any user prompt. The app itself is designed to handle deep links (such as opening an album or starting the song).<p>That means you can perform a deep link forgery, in order to force the app to perform unwilling action without user confirmation.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151621&amp;goto=item%3Fid%3D27145911%2327151621">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147634'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147634' onclick='return vote(event, this, "up")' href='vote?id=27147634&amp;how=up&amp;auth=3bb272801f9ffe33f069746b8f3d06c021ce5199&amp;goto=item%3Fid%3D27145911#27147634'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Forbo" class="hnuser">Forbo</a> <span class="age"><a href="item?id=27147634">2 days ago</a></span> <span id="unv_27147634"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147634)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I appear to be getting false positives with a different identifier each time I run it. It says I have 3-4 different applications installed, none of which actually are on my system. Each subsequent run comes back with a different set of applications, and a different unique identifier. Looks like I may have beaten this method of fingerprinting, although I&#x27;m not quite sure how.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147634&amp;goto=item%3Fid%3D27145911%2327147634">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148460'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148460' onclick='return vote(event, this, "up")' href='vote?id=27148460&amp;how=up&amp;auth=49f4861cf44e786e860cc7c9dd1b114538b5b7df&amp;goto=item%3Fid%3D27145911#27148460'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=antpls" class="hnuser">antpls</a> <span class="age"><a href="item?id=27148460">2 days ago</a></span> <span id="unv_27148460"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148460)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Could be alleviated by creating yet another permission at the browser level : &quot;allow to link to local applications&quot;</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148460&amp;goto=item%3Fid%3D27145911%2327148460">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148992'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27148992' onclick='return vote(event, this, "up")' href='vote?id=27148992&amp;how=up&amp;auth=333d923103dcde6501fc947527c4fc822d04f62a&amp;goto=item%3Fid%3D27145911#27148992'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=livre" class="hnuser">livre</a> <span class="age"><a href="item?id=27148992">2 days ago</a></span> <span id="unv_27148992"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148992)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Firefox on Android has a setting &quot;Open links in apps&quot; and works similar to the way you describe but it&#x27;s global, either enabled for all websites or disabled for all. I agree that something similar on desktop would be useful.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148992&amp;goto=item%3Fid%3D27145911%2327148992">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147954'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147954' onclick='return vote(event, this, "up")' href='vote?id=27147954&amp;how=up&amp;auth=ec13cd579ee05bc3b7e040e4006478abce6fa49f&amp;goto=item%3Fid%3D27145911#27147954'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kofejnik" class="hnuser">kofejnik</a> <span class="age"><a href="item?id=27147954">2 days ago</a></span> <span id="unv_27147954"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147954)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Confirmed - my ID matched in Chrome and Safari, but Firefox just said 24 of 24 and gave a different ID. Firefox wins again!</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147954&amp;goto=item%3Fid%3D27145911%2327147954">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148071'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148071' onclick='return vote(event, this, "up")' href='vote?id=27148071&amp;how=up&amp;auth=be2b25722bb9644f6d6d5b78e20925bb716f8e88&amp;goto=item%3Fid%3D27145911#27148071'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Jap2-0" class="hnuser">Jap2-0</a> <span class="age"><a href="item?id=27148071">2 days ago</a></span> <span id="unv_27148071"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148071)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">On Firefox on Windows (same results on Edge) it detected three programs I do have installed, and one I do not, and failed to detect one I do have installed.  There was a moderately noticeable small window in the bottom right of the screen in both.<p>That said, at least for tracking consistency is more important than accuracy.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148071&amp;goto=item%3Fid%3D27145911%2327148071">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148526'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148526' onclick='return vote(event, this, "up")' href='vote?id=27148526&amp;how=up&amp;auth=e2b343481a4c98845cbbd6c7678b7c4c4f6610fb&amp;goto=item%3Fid%3D27145911#27148526'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Isthatablackgsd" class="hnuser">Isthatablackgsd</a> <span class="age"><a href="item?id=27148526">2 days ago</a></span> <span id="unv_27148526"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148526)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It seem that Vivaldi have better protection against this than the rest. Running in Vivaldi will cause the demo down to crawl because I think it was trying to find the apps. It detected all of the apps but it failed to appear in the detected list. MacOS Big Sur Apple Silicon if you are wondering</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148526&amp;goto=item%3Fid%3D27145911%2327148526">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151632'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27151632' onclick='return vote(event, this, "up")' href='vote?id=27151632&amp;how=up&amp;auth=d30ed4b2dd73088ca279588be9d5cb007b13caef&amp;goto=item%3Fid%3D27145911#27151632'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27151632">2 days ago</a></span> <span id="unv_27151632"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151632)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">We haven&#x27;t tested Vivaldi so far and the demo is not designed for it. However that doesn&#x27;t mean Vivaldi is secure against this attack.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151632&amp;goto=item%3Fid%3D27145911%2327151632">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27148877'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148877' onclick='return vote(event, this, "up")' href='vote?id=27148877&amp;how=up&amp;auth=36b1ebb4cf2a9d37ac28a254def9f61947c4721d&amp;goto=item%3Fid%3D27145911#27148877'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=busymom0" class="hnuser">busymom0</a> <span class="age"><a href="item?id=27148877">2 days ago</a></span> <span id="unv_27148877"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148877)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c5a">&gt; Safari: Despite privacy being a main development focus for the Safari browser, it turned out to be the easiest browser of the four to exploit. Safari doesn’t have scheme flood protection which allows the exploit to easily enumerate all installed applications. The same-origin policy trick as used for the Firefox browser was used here as well.<p>On iOS (and MacOS too I believe), when developing apps and requesting URL scheme, the developer has to declare every URL scheme they want their app to query in the `LSApplicationQueriesSchemes` array in info.plist of the app. This was added in iOS 9 as part of a similar vulnerability where apps and advertisement SDKs would simply query a list of URL schemes and then identify based on that across multiple apps. Something similar can be done by browsers for websites. MacOS could simply show you a popup with a checkbox list of all URL schemes the site tries to query for with options for &quot;Allow&quot;, &quot;Deny&quot; or &quot;Randomize&quot;.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148877&amp;goto=item%3Fid%3D27145911%2327148877">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148194'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148194' onclick='return vote(event, this, "up")' href='vote?id=27148194&amp;how=up&amp;auth=2dcd16f108029393074e04623104e9f9401869f6&amp;goto=item%3Fid%3D27145911#27148194'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=hirsin" class="hnuser">hirsin</a> <span class="age"><a href="item?id=27148194">2 days ago</a></span> <span id="unv_27148194"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148194)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">My searching is failing, but I believe a similar scheme was uncovered in a popular app using a &#x27;strings&#x27; equivalent. It would run through intents on iOS and Android to figure out what was installed. Interesting to see if on the web too!</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148194&amp;goto=item%3Fid%3D27145911%2327148194">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27150409'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27150409' onclick='return vote(event, this, "up")' href='vote?id=27150409&amp;how=up&amp;auth=b968b7e99a695287393e8c0f2794b3dce7cf4b5b&amp;goto=item%3Fid%3D27145911#27150409'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=lilyball" class="hnuser">lilyball</a> <span class="age"><a href="item?id=27150409">2 days ago</a></span> <span id="unv_27150409"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27150409)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Why does this page declare Safari to be the easiest browser to exploit, when it also says it uses the same technique as it does in Firefox (and does not describe any scheme flooding protection in Firefox)?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150409&amp;goto=item%3Fid%3D27145911%2327150409">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151576'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27151576' onclick='return vote(event, this, "up")' href='vote?id=27151576&amp;how=up&amp;auth=e691e18a797764146f3a2a5d220aaaba6c65876c&amp;goto=item%3Fid%3D27145911#27151576'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27151576">2 days ago</a></span> <span id="unv_27151576"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151576)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Mostly because it took hours to make an exploit on Safari compared to days on Firefox, however the final approach ended up the same. Only Chromium has a built-in scheme anti-flooding protection.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151576&amp;goto=item%3Fid%3D27145911%2327151576">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27148841'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148841' onclick='return vote(event, this, "up")' href='vote?id=27148841&amp;how=up&amp;auth=dff9470cc442378633ddfd1fb89bb94f8f137271&amp;goto=item%3Fid%3D27145911#27148841'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=jonnycomputer" class="hnuser">jonnycomputer</a> <span class="age"><a href="item?id=27148841">2 days ago</a></span> <span id="unv_27148841"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148841)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">First time had 2 apps that aren&#x27;t installed. Second two attempts didn&#x27;t detect those two apps.<p>I didn&#x27;t even notice the window flashing the first time round.<p>I&#x27;d say that this is a reasonably serious privacy vulnerability.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148841&amp;goto=item%3Fid%3D27145911%2327148841">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147635'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147635' onclick='return vote(event, this, "up")' href='vote?id=27147635&amp;how=up&amp;auth=28d61dcfd0162cc3c06d04e14cfc0b8b408667d9&amp;goto=item%3Fid%3D27145911#27147635'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=tn1" class="hnuser">tn1</a> <span class="age"><a href="item?id=27147635">2 days ago</a></span> <span id="unv_27147635"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147635)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I tried it on Opera and it detected no apps installed. (On Edge however, it detects all the ones I do indeed have installed).<p>This is interesting since I didn&#x27;t really expect Opera to care about this kind of thing.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147635&amp;goto=item%3Fid%3D27145911%2327147635">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147669'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147669' onclick='return vote(event, this, "up")' href='vote?id=27147669&amp;how=up&amp;auth=8da1b14821f96578d0110fbfb4aa412b894a26d3&amp;goto=item%3Fid%3D27145911#27147669'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=valve1" class="hnuser">valve1</a> <span class="age"><a href="item?id=27147669">2 days ago</a></span> <span id="unv_27147669"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147669)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Thanks for testing this on Opera, we only tested on these browser&#x2F;OS combinations: <a href="https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding#target-browsers" rel="nofollow">https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding#...</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147669&amp;goto=item%3Fid%3D27145911%2327147669">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27148424'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148424' onclick='return vote(event, this, "up")' href='vote?id=27148424&amp;how=up&amp;auth=263fbf34b20379ec77233178cb5db59b6f7bccb0&amp;goto=item%3Fid%3D27145911#27148424'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=bryan_w" class="hnuser">bryan_w</a> <span class="age"><a href="item?id=27148424">2 days ago</a></span> <span id="unv_27148424"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148424)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Seems like this submission is a bit undercooked. It probably should have been submitted once they had some real world samples or at least gated it to their specific use case</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148424&amp;goto=item%3Fid%3D27145911%2327148424">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147670'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147670' onclick='return vote(event, this, "up")' href='vote?id=27147670&amp;how=up&amp;auth=d8e52141b15519cfe9922f07c956a0a30b8fe1e2&amp;goto=item%3Fid%3D27145911#27147670'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=johnvaluk" class="hnuser">johnvaluk</a> <span class="age"><a href="item?id=27147670">2 days ago</a></span> <span id="unv_27147670"></span><span class="par"></span> <a class="togg" n="6" href="javascript:void(0)" onclick="return toggle(event, 27147670)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">This appears to depend on user interactivity. How would you silently (and accurately) use this technique to fingerprint a system for cross-browser tracking?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147670&amp;goto=item%3Fid%3D27145911%2327147670">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147739'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147739' onclick='return vote(event, this, "up")' href='vote?id=27147739&amp;how=up&amp;auth=fbdb30e0227bb000bfd3c286c6e90d0488caa2cf&amp;goto=item%3Fid%3D27145911#27147739'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=shadowgovt" class="hnuser">shadowgovt</a> <span class="age"><a href="item?id=27147739">2 days ago</a></span> <span id="unv_27147739"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147739)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It would be trickier, but it&#x27;s not as hard as one might want to get a user to click in such a way that the protections in place against automated behaviors can be side-stepped.<p>I&#x27;d bet good money that this trick would be useful for anyone running either a meme generator website or a file host, for example. It&#x27;d be pretty solid in the file host in particular, because you could hide some of the obvious weird behavior behind the &quot;We&#x27;re downloading your file&quot; delay.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147739&amp;goto=item%3Fid%3D27145911%2327147739">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27147706'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147706' onclick='return vote(event, this, "up")' href='vote?id=27147706&amp;how=up&amp;auth=f21b267273561432a6fd5984c50a2fa10a35cf95&amp;goto=item%3Fid%3D27145911#27147706'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=valve1" class="hnuser">valve1</a> <span class="age"><a href="item?id=27147706">2 days ago</a></span> <span id="unv_27147706"></span><span class="par"></span> <a class="togg" n="4" href="javascript:void(0)" onclick="return toggle(event, 27147706)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">On Tor we show a fake captcha on the demo, which allows to collect multiple key presses and use each as a user-provided trigger.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147706&amp;goto=item%3Fid%3D27145911%2327147706">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147839'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147839' onclick='return vote(event, this, "up")' href='vote?id=27147839&amp;how=up&amp;auth=65232f977225788a5025397c6d1d633d1fcd9929&amp;goto=item%3Fid%3D27145911#27147839'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Ansil849" class="hnuser">Ansil849</a> <span class="age"><a href="item?id=27147839">2 days ago</a></span> <span id="unv_27147839"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147839)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">This is a really clever way to coerce interactivity!</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147839&amp;goto=item%3Fid%3D27145911%2327147839">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148975'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27148975' onclick='return vote(event, this, "up")' href='vote?id=27148975&amp;how=up&amp;auth=e247140aa49ebfd27415eebd8f4a5bce47fb21e1&amp;goto=item%3Fid%3D27145911#27148975'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=nulbyte" class="hnuser">nulbyte</a> <span class="age"><a href="item?id=27148975">2 days ago</a></span> <span id="unv_27148975"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148975)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Clever indeed; I only suspected this the second go-round, after I noticed the reload button flicker as I typed. I also noticed you don&#x27;t have to press Enter or even type the correct phrase to get past the fake prompt. In hindsight, the easy to guess text should have been a dead give-away, but it wasn&#x27;t.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148975&amp;goto=item%3Fid%3D27145911%2327148975">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27147965'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147965' onclick='return vote(event, this, "up")' href='vote?id=27147965&amp;how=up&amp;auth=8d8f9f9f92d78270b68450a37731063f0b927a6e&amp;goto=item%3Fid%3D27145911#27147965'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=johnvaluk" class="hnuser">johnvaluk</a> <span class="age"><a href="item?id=27147965">2 days ago</a></span> <span id="unv_27147965"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147965)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Does that bypass any alerts that would be presented to the user by the browser?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147965&amp;goto=item%3Fid%3D27145911%2327147965">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                      <tr class='athing comtr' id='27147230'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147230' onclick='return vote(event, this, "up")' href='vote?id=27147230&amp;how=up&amp;auth=ace6830bfaa82c28e20e007db41f7b7523aa72f3&amp;goto=item%3Fid%3D27145911#27147230'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=bryanrasmussen" class="hnuser">bryanrasmussen</a> <span class="age"><a href="item?id=27147230">2 days ago</a></span> <span id="unv_27147230"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147230)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">This seems less promising as a means to uniquely identify users than supercookies, Time-Based Device Fingerprinting, or other hardware based methods.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147230&amp;goto=item%3Fid%3D27145911%2327147230">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148849'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148849' onclick='return vote(event, this, "up")' href='vote?id=27148849&amp;how=up&amp;auth=d8102b4e32d3cece274882ea51a41f972aa193b0&amp;goto=item%3Fid%3D27145911#27148849'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=eithed" class="hnuser">eithed</a> <span class="age"><a href="item?id=27148849">2 days ago</a></span> <span id="unv_27148849"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148849)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Got Word, Discord, Slack and Postman installed, yet those were not detected. Chrome on Windows.<p>All, except Word, were succesfully detected in Firefox though</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148849&amp;goto=item%3Fid%3D27145911%2327148849">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27146960'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27146960' onclick='return vote(event, this, "up")' href='vote?id=27146960&amp;how=up&amp;auth=42c7f23d4674e9cf19cff1551d0dd959f4f95398&amp;goto=item%3Fid%3D27145911#27146960'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=agilob" class="hnuser">agilob</a> <span class="age"><a href="item?id=27146960">2 days ago</a></span> <span id="unv_27146960"></span><span class="par"></span> <a class="togg" n="4" href="javascript:void(0)" onclick="return toggle(event, 27146960)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Tried Chrome, Brave and Firefox, got 3 different IDs.<p>On one of the browsers it also didn&#x27;t detect slack and vscode being installed.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27146960&amp;goto=item%3Fid%3D27145911%2327146960">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147352'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147352' onclick='return vote(event, this, "up")' href='vote?id=27147352&amp;how=up&amp;auth=a54ad3251653fa19311d65bb185cb6904d72a104&amp;goto=item%3Fid%3D27145911#27147352'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27147352">2 days ago</a></span> <span id="unv_27147352"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147352)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Hi, agilob. I&#x27;ve updated the demo for Chromium and made it work slower, in order to increase accuracy. See also <a href="https:&#x2F;&#x2F;news.ycombinator.com&#x2F;item?id=27147325" rel="nofollow">https:&#x2F;&#x2F;news.ycombinator.com&#x2F;item?id=27147325</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147352&amp;goto=item%3Fid%3D27145911%2327147352">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27152351'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27152351' onclick='return vote(event, this, "up")' href='vote?id=27152351&amp;how=up&amp;auth=e3b9de69ede811eba089f6919ef01dc4de80d253&amp;goto=item%3Fid%3D27145911#27152351'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=agilob" class="hnuser">agilob</a> <span class="age"><a href="item?id=27152351">2 days ago</a></span> <span id="unv_27152351"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27152351)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Now I got an identifier that you saw 2 times before:<p>&gt;This is your identifier. It was seen 2 times among 8828 tests so far.<p>None of these was my run. Still, didn&#x27;t detect vscode :)</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27152351&amp;goto=item%3Fid%3D27145911%2327152351">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27147343'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147343' onclick='return vote(event, this, "up")' href='vote?id=27147343&amp;how=up&amp;auth=96a1a87ab3216f55c87749835362a906b58c0694&amp;goto=item%3Fid%3D27145911#27147343'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dathinab" class="hnuser">dathinab</a> <span class="age"><a href="item?id=27147343">2 days ago</a></span> <span id="unv_27147343"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147343)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">&gt; didn&#x27;t detect slack and vscode being installed.<p>Is it you main browser in which you had used slack url&#x27;s&#x2F; set slack to always handle the links?<p>Or is it the opposite?<p>Or maybe something else?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147343&amp;goto=item%3Fid%3D27145911%2327147343">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147873'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147873' onclick='return vote(event, this, "up")' href='vote?id=27147873&amp;how=up&amp;auth=db2e332afe2e0e5971581e70b911d0b00a2440e3&amp;goto=item%3Fid%3D27145911#27147873'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=wiiittttt" class="hnuser">wiiittttt</a> <span class="age"><a href="item?id=27147873">2 days ago</a></span> <span id="unv_27147873"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147873)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I received different results in Firefox and Brave. Doesn&#x27;t seem to be a reliable method for tracking.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147873&amp;goto=item%3Fid%3D27145911%2327147873">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147373'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147373' onclick='return vote(event, this, "up")' href='vote?id=27147373&amp;how=up&amp;auth=5fbb0a08af2233a34a63a4b73a389d94a54ba623&amp;goto=item%3Fid%3D27145911#27147373'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=conradev" class="hnuser">conradev</a> <span class="age"><a href="item?id=27147373">2 days ago</a></span> <span id="unv_27147373"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147373)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c5a">Finding new a fingerprinting mechanism in JavaScript is like finding a new memory corruption bug in the web browser engine.<p>They are always going to exist for architectural reasons, some are worse than others, and the really bad ones are likely kept nice and secret while they are actively exploited. In other words, I&#x27;m not surprised in the slightest, but I&#x27;m glad that this is out in the open now.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147373&amp;goto=item%3Fid%3D27145911%2327147373">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27149753'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27149753' onclick='return vote(event, this, "up")' href='vote?id=27149753&amp;how=up&amp;auth=12b59596d63affdb72b63f36e2dd36b3d7589c2e&amp;goto=item%3Fid%3D27145911#27149753'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=zerof1l" class="hnuser">zerof1l</a> <span class="age"><a href="item?id=27149753">2 days ago</a></span> <span id="unv_27149753"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27149753)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Tested on Windows and got two different identifiers. Firefox detected Postman, Chrome did not.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149753&amp;goto=item%3Fid%3D27145911%2327149753">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151640'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27151640' onclick='return vote(event, this, "up")' href='vote?id=27151640&amp;how=up&amp;auth=dd4d72bbc573a93edf62984a65f52f8b82c61906&amp;goto=item%3Fid%3D27145911#27151640'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27151640">2 days ago</a></span> <span id="unv_27151640"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151640)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Thanks for the feedback. The accuracy is the main issue on Chrome. See also <a href="https:&#x2F;&#x2F;news.ycombinator.com&#x2F;item?id=27147876" rel="nofollow">https:&#x2F;&#x2F;news.ycombinator.com&#x2F;item?id=27147876</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151640&amp;goto=item%3Fid%3D27145911%2327151640">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147927'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147927' onclick='return vote(event, this, "up")' href='vote?id=27147927&amp;how=up&amp;auth=f015dd825f581b96dbf65469d0a751f455fca1e1&amp;goto=item%3Fid%3D27145911#27147927'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kjrose" class="hnuser">kjrose</a> <span class="age"><a href="item?id=27147927">2 days ago</a></span> <span id="unv_27147927"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147927)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c5a">As a note, this doesn&#x27;t seem to work with Brave. It only got one of the applications my machine has installed, and I don&#x27;t have a slow machine nor a slow internet where I am.<p>I&#x27;m a bit surprised it got even one of them though. I will need to review my Brave privacy settings and see if anything can be done.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147927&amp;goto=item%3Fid%3D27145911%2327147927">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147992'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147992' onclick='return vote(event, this, "up")' href='vote?id=27147992&amp;how=up&amp;auth=df033b3d7c73eabf5528475904183420291506af&amp;goto=item%3Fid%3D27145911#27147992'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=pier25" class="hnuser">pier25</a> <span class="age"><a href="item?id=27147992">2 days ago</a></span> <span id="unv_27147992"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147992)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I just tried it with the latest version of Brave and it found: Skype, Zoom, VSCode, Adobe, and iTunes.<p>This only checks 24 apps, and it got all the ones I have installed out of those 24.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147992&amp;goto=item%3Fid%3D27145911%2327147992">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27147376'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147376' onclick='return vote(event, this, "up")' href='vote?id=27147376&amp;how=up&amp;auth=5a2eab5b30b48480218f325f6c4400009ba42d6c&amp;goto=item%3Fid%3D27145911#27147376'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=wnevets" class="hnuser">wnevets</a> <span class="age"><a href="item?id=27147376">2 days ago</a></span> <span id="unv_27147376"></span><span class="par"></span> <a class="togg" n="6" href="javascript:void(0)" onclick="return toggle(event, 27147376)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It though I had Skype, Spotify and Slack installed. I only have Slack installed.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147376&amp;goto=item%3Fid%3D27145911%2327147376">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147399'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147399' onclick='return vote(event, this, "up")' href='vote?id=27147399&amp;how=up&amp;auth=8417d3260fc06a2703e802133a3a1696237e89fc&amp;goto=item%3Fid%3D27145911#27147399'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=valve1" class="hnuser">valve1</a> <span class="age"><a href="item?id=27147399">2 days ago</a></span> <span id="unv_27147399"></span><span class="par"></span> <a class="togg" n="5" href="javascript:void(0)" onclick="return toggle(event, 27147399)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Windows can sometimes say you have Skype, because it comes bundled even if you didn&#x27;t install it yourself.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147399&amp;goto=item%3Fid%3D27145911%2327147399">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147447'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147447' onclick='return vote(event, this, "up")' href='vote?id=27147447&amp;how=up&amp;auth=c19b32fdbf6ec22178b54c6c0613c37c2990328c&amp;goto=item%3Fid%3D27145911#27147447'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=wnevets" class="hnuser">wnevets</a> <span class="age"><a href="item?id=27147447">2 days ago</a></span> <span id="unv_27147447"></span><span class="par"></span> <a class="togg" n="4" href="javascript:void(0)" onclick="return toggle(event, 27147447)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I&#x27;ve explicitly uninstalled it on Windows 10, maybe Windows is still reporting it?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147447&amp;goto=item%3Fid%3D27145911%2327147447">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147880'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27147880' onclick='return vote(event, this, "up")' href='vote?id=27147880&amp;how=up&amp;auth=222c739de315ad5f167c4a3668b92eea6b2f2688&amp;goto=item%3Fid%3D27145911#27147880'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=tick_tock_tick" class="hnuser">tick_tock_tick</a> <span class="age"><a href="item?id=27147880">2 days ago</a></span> <span id="unv_27147880"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27147880)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Windows 10 does some garbage where it installs handlers for URL schemas that take you to the windows store install page for the app. The vulnerability is only testing if you have an handler installed for skype:&#x2F;&#x2F; not what application is actually handling it.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147880&amp;goto=item%3Fid%3D27145911%2327147880">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27149238'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="160"></td><td valign="top" class="votelinks">
          <center><a id='up_27149238' onclick='return vote(event, this, "up")' href='vote?id=27149238&amp;how=up&amp;auth=876c5d7436f9acb72443bb39c4a6bb0d1b2cd467&amp;goto=item%3Fid%3D27145911#27149238'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=wnevets" class="hnuser">wnevets</a> <span class="age"><a href="item?id=27149238">2 days ago</a></span> <span id="unv_27149238"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149238)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">According to URLProtocolView[1] the handlers are still registered despite the application (and MANY others) being uninstalled.<p>[1] <a href="https:&#x2F;&#x2F;www.nirsoft.net&#x2F;utils&#x2F;url_protocol_view.html" rel="nofollow">https:&#x2F;&#x2F;www.nirsoft.net&#x2F;utils&#x2F;url_protocol_view.html</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149238&amp;goto=item%3Fid%3D27145911%2327149238">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27148099'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="160"></td><td valign="top" class="votelinks">
          <center><a id='up_27148099' onclick='return vote(event, this, "up")' href='vote?id=27148099&amp;how=up&amp;auth=d3f1bd4d75c4edca8056f374281779e1923289b7&amp;goto=item%3Fid%3D27145911#27148099'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=wnevets" class="hnuser">wnevets</a> <span class="age"><a href="item?id=27148099">2 days ago</a></span> <span id="unv_27148099"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148099)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Windows 10 must be doing something weird. Skype url handlers aren&#x27;t triggering the window stores or anything else from links.<p><a href="https:&#x2F;&#x2F;jsfiddle.net&#x2F;ourcodeworld&#x2F;aqq1w0qm&#x2F;" rel="nofollow">https:&#x2F;&#x2F;jsfiddle.net&#x2F;ourcodeworld&#x2F;aqq1w0qm&#x2F;</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148099&amp;goto=item%3Fid%3D27145911%2327148099">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                                  <tr class='athing comtr' id='27147008'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147008' onclick='return vote(event, this, "up")' href='vote?id=27147008&amp;how=up&amp;auth=2c4ba25127439c0c003f2f9e0fe6c3961d8c0cb7&amp;goto=item%3Fid%3D27145911#27147008'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Operyl" class="hnuser">Operyl</a> <span class="age"><a href="item?id=27147008">2 days ago</a></span> <span id="unv_27147008"></span><span class="par"></span> <a class="togg" n="5" href="javascript:void(0)" onclick="return toggle(event, 27147008)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It&#x27;s not detecting many of the supported applications on my Mac in Safari.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147008&amp;goto=item%3Fid%3D27145911%2327147008">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147313'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147313' onclick='return vote(event, this, "up")' href='vote?id=27147313&amp;how=up&amp;auth=a699f4139c4bc24edbe2dcb5c0d0c4addd44d629&amp;goto=item%3Fid%3D27145911#27147313'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27147313">2 days ago</a></span> <span id="unv_27147313"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27147313)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">The exploit was tested in Safari 14.0.3 and 14.1 on MacBook M1 and MacBook Pro. What version do you have?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147313&amp;goto=item%3Fid%3D27145911%2327147313">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27148883'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27148883' onclick='return vote(event, this, "up")' href='vote?id=27148883&amp;how=up&amp;auth=e83d08331c1192f00933ff8ef3746c29a07bcdc6&amp;goto=item%3Fid%3D27145911#27148883'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Operyl" class="hnuser">Operyl</a> <span class="age"><a href="item?id=27148883">2 days ago</a></span> <span id="unv_27148883"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148883)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">14.1 on an M1 MBP.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148883&amp;goto=item%3Fid%3D27145911%2327148883">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27151660'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="120"></td><td valign="top" class="votelinks">
          <center><a id='up_27151660' onclick='return vote(event, this, "up")' href='vote?id=27151660&amp;how=up&amp;auth=22829c4d733253c634be239bf72ce9d096ee6806&amp;goto=item%3Fid%3D27145911#27151660'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27151660">2 days ago</a></span> <span id="unv_27151660"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151660)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Wow, that&#x27;s weird.<p>The internet connection may be the issue here, or the custom configuration on Safari.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151660&amp;goto=item%3Fid%3D27145911%2327151660">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                            <tr class='athing comtr' id='27147089'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147089' onclick='return vote(event, this, "up")' href='vote?id=27147089&amp;how=up&amp;auth=86d79e108cabda956a0908a4cb6b1d0d6ee7ca66&amp;goto=item%3Fid%3D27145911#27147089'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=dzhiurgis" class="hnuser">dzhiurgis</a> <span class="age"><a href="item?id=27147089">2 days ago</a></span> <span id="unv_27147089"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147089)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It did for me and compared with Chrome identified everything to same identifier.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147089&amp;goto=item%3Fid%3D27145911%2327147089">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27146953'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27146953' onclick='return vote(event, this, "up")' href='vote?id=27146953&amp;how=up&amp;auth=c7f6ddb5e12a1e3ef7187f250c822f55d156eefe&amp;goto=item%3Fid%3D27145911#27146953'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=jowsie" class="hnuser">jowsie</a> <span class="age"><a href="item?id=27146953">2 days ago</a></span> <span id="unv_27146953"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27146953)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I ran this in Chrome and then in Edge and got different identifiers.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27146953&amp;goto=item%3Fid%3D27145911%2327146953">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147325'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147325' onclick='return vote(event, this, "up")' href='vote?id=27147325&amp;how=up&amp;auth=777834be50a59b9dc292ded911158d63d95ecbc4&amp;goto=item%3Fid%3D27145911#27147325'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27147325">2 days ago</a></span> <span id="unv_27147325"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147325)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Chromium results may be flaky on slow internet or because of less performant hardware (such as Virtual Machines).<p>I&#x27;ve updated the demo for Chromium and made it work slower, in order to increase accuracy.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147325&amp;goto=item%3Fid%3D27145911%2327147325">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                                <tr class='athing comtr' id='27148509'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148509' onclick='return vote(event, this, "up")' href='vote?id=27148509&amp;how=up&amp;auth=e93028f1671d2e3e8a08e8e739e7aba5d1a658c3&amp;goto=item%3Fid%3D27145911#27148509'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=edoceo" class="hnuser">edoceo</a> <span class="age"><a href="item?id=27148509">2 days ago</a></span> <span id="unv_27148509"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148509)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Only one right answer on my machine - that&#x27;s ~5% accurate.<p>Linux&#x2F;Chrome</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148509&amp;goto=item%3Fid%3D27145911%2327148509">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147566'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147566' onclick='return vote(event, this, "up")' href='vote?id=27147566&amp;how=up&amp;auth=fdaa5a58bf4902f905b785edfe901837d783c9d7&amp;goto=item%3Fid%3D27145911#27147566'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=butz" class="hnuser">butz</a> <span class="age"><a href="item?id=27147566">2 days ago</a></span> <span id="unv_27147566"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147566)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Test doesn&#x27;t work when localStorage is disabled in browser.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147566&amp;goto=item%3Fid%3D27145911%2327147566">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147721'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147721' onclick='return vote(event, this, "up")' href='vote?id=27147721&amp;how=up&amp;auth=01b8c10aee73d27877fea4eb937e2400995f35e1&amp;goto=item%3Fid%3D27145911#27147721'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=anon776" class="hnuser">anon776</a> <span class="age"><a href="item?id=27147721">2 days ago</a></span> <span id="unv_27147721"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147721)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Anyone try this with tails&#x2F;tor? how unique were they?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147721&amp;goto=item%3Fid%3D27145911%2327147721">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148690'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148690' onclick='return vote(event, this, "up")' href='vote?id=27148690&amp;how=up&amp;auth=a25205522dc89d62a1b9c829443b8b171b98a9e4&amp;goto=item%3Fid%3D27145911#27148690'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=throwawayay02" class="hnuser">throwawayay02</a> <span class="age"><a href="item?id=27148690">2 days ago</a></span> <span id="unv_27148690"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148690)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">On my 86.0.1 firefox on linux it detected nothing.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148690&amp;goto=item%3Fid%3D27145911%2327148690">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27148661'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27148661' onclick='return vote(event, this, "up")' href='vote?id=27148661&amp;how=up&amp;auth=68a875177f59135dc85d8852d9f1420d34f151de&amp;goto=item%3Fid%3D27145911#27148661'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=usmannk" class="hnuser">usmannk</a> <span class="age"><a href="item?id=27148661">2 days ago</a></span> <span id="unv_27148661"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148661)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">hm, I tried this on 2 browsers and it said I was unique both times. Shouldn&#x27;t the second have been a collision?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27148661&amp;goto=item%3Fid%3D27145911%2327148661">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27149082'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27149082' onclick='return vote(event, this, "up")' href='vote?id=27149082&amp;how=up&amp;auth=5f019db6f256056a4b05c87eb76c1c839d325a37&amp;goto=item%3Fid%3D27145911#27149082'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=afrcnc" class="hnuser">afrcnc</a> <span class="age"><a href="item?id=27149082">2 days ago</a></span> <span id="unv_27149082"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27149082)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Old trick and demo doesn&#x27;t work :)</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27149082&amp;goto=item%3Fid%3D27145911%2327149082">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147609'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147609' onclick='return vote(event, this, "up")' href='vote?id=27147609&amp;how=up&amp;auth=dc95e4f1dd2f4e5d8c75e93eab8a27a493e63caa&amp;goto=item%3Fid%3D27145911#27147609'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=taf2" class="hnuser">taf2</a> <span class="age"><a href="item?id=27147609">2 days ago</a></span> <span id="unv_27147609"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147609)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Interesting but only works on desktop</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147609&amp;goto=item%3Fid%3D27145911%2327147609">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27150010'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27150010' onclick='return vote(event, this, "up")' href='vote?id=27150010&amp;how=up&amp;auth=11c36b67c22eb6805e0dd5afbb763b77f49594e4&amp;goto=item%3Fid%3D27145911#27150010'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=userbinator" class="hnuser">userbinator</a> <span class="age"><a href="item?id=27150010">2 days ago</a></span> <span id="unv_27150010"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27150010)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">No mention of IE...</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27150010&amp;goto=item%3Fid%3D27145911%2327150010">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27151244'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27151244' onclick='return vote(event, this, "up")' href='vote?id=27151244&amp;how=up&amp;auth=a9ea7de38cb2bf51a209726443e22f34eb9a42d3&amp;goto=item%3Fid%3D27145911#27151244'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=est" class="hnuser">est</a> <span class="age"><a href="item?id=27151244">2 days ago</a></span> <span id="unv_27151244"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27151244)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c73">Browsers should by default &lt;body onload=&quot;disableJS()&quot;&gt;</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27151244&amp;goto=item%3Fid%3D27145911%2327151244">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147788'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147788' onclick='return vote(event, this, "up")' href='vote?id=27147788&amp;how=up&amp;auth=d647c831b44f87fbb80f672bf94f2dc934aeb50e&amp;goto=item%3Fid%3D27145911#27147788'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Ansil849" class="hnuser">Ansil849</a> <span class="age"><a href="item?id=27147788">2 days ago</a></span> <span id="unv_27147788"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147788)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c73">Visiting the demo website in Tor Browser (using the &#x27;Safest&#x27; setting), the demo site displays this notice:<p>&gt; If you&#x27;re seeing this message, that means JavaScript has been disabled on your browser, please enable JS to make this app work.<p>Does this mean that the vulnerability does not work in Tor Browser in Safest mode? Or are there non-JS implementations of this vulnerability that would work in a browser with JS disabled?</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147788&amp;goto=item%3Fid%3D27145911%2327147788">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          <tr class='athing comtr' id='27147196'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="0"></td><td valign="top" class="votelinks">
          <center><a id='up_27147196' onclick='return vote(event, this, "up")' href='vote?id=27147196&amp;how=up&amp;auth=b0a47e7982040e43b60348061963096793cd8cce&amp;goto=item%3Fid%3D27145911#27147196'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=rozab" class="hnuser">rozab</a> <span class="age"><a href="item?id=27147196">2 days ago</a></span> <span id="unv_27147196"></span><span class="par"></span> <a class="togg" n="9" href="javascript:void(0)" onclick="return toggle(event, 27147196)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c73">Does this actually work correctly for anyone? Got wrong results for Firefox and Chrome on Linux (it warns that Chrome probably won&#x27;t work).<p>I glanced through the source[0] and my about:config and I noticed I have the dom.block_external_protocol_in_iframes setting enabled. Looks like this could be the mechanism they use? I don&#x27;t remember enabling it manually.<p>Otherwise, it could be my tiling window manager messing with detection.<p>[0]: <a href="https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding&#x2F;blob&#x2F;3076ec617f5e5ab1ad72642898ca01f40271cf41&#x2F;packages&#x2F;client&#x2F;src&#x2F;detector&#x2F;detection.ts#L222" rel="nofollow">https:&#x2F;&#x2F;github.com&#x2F;fingerprintjs&#x2F;external-protocol-flooding&#x2F;...</a></span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147196&amp;goto=item%3Fid%3D27145911%2327147196">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147254'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147254' onclick='return vote(event, this, "up")' href='vote?id=27147254&amp;how=up&amp;auth=26a9c0d31081ead30ecebd42d85359e83cc5f88a&amp;goto=item%3Fid%3D27145911#27147254'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kdarutkin" class="hnuser">kdarutkin</a> <span class="age"><a href="item?id=27147254">2 days ago</a></span> <span id="unv_27147254"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147254)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Any custom settings may affect the result. However default settings will work for the Firefox 88.0.1. Was tested on Windows, Safari and Linux.<p>Chrome does not work on Ubuntu, since it opens everything with xdg-open and creates confirmation dialog for both installed and not-installed application</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147254&amp;goto=item%3Fid%3D27145911%2327147254">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27147629'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147629' onclick='return vote(event, this, "up")' href='vote?id=27147629&amp;how=up&amp;auth=d02e73f59a29e85cbe5e7d48ae914384c4746267&amp;goto=item%3Fid%3D27145911#27147629'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=eulers_secret" class="hnuser">eulers_secret</a> <span class="age"><a href="item?id=27147629">2 days ago</a></span> <span id="unv_27147629"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147629)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Worked for me on FF 88.0&#x2F;Kubuntu 21.04. Detected the 2 apps I have installed correctly. I was also unique.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147629&amp;goto=item%3Fid%3D27145911%2327147629">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27147298'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147298' onclick='return vote(event, this, "up")' href='vote?id=27147298&amp;how=up&amp;auth=456666ed27e3b2fc90a10e3dd5bb9959cf67c602&amp;goto=item%3Fid%3D27145911#27147298'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=kurthr" class="hnuser">kurthr</a> <span class="age"><a href="item?id=27147298">2 days ago</a></span> <span id="unv_27147298"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27147298)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">I find it interesting that it shows I have Skype installed... when I don&#x27;t.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147298&amp;goto=item%3Fid%3D27145911%2327147298">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147791'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147791' onclick='return vote(event, this, "up")' href='vote?id=27147791&amp;how=up&amp;auth=4aebad29ab4ca485cea1bfd1c850e60a0c66ccde&amp;goto=item%3Fid%3D27145911#27147791'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=shadowgovt" class="hnuser">shadowgovt</a> <span class="age"><a href="item?id=27147791">2 days ago</a></span> <span id="unv_27147791"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147791)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Do you remember ever having Skype installed? Sibling comments suggest that some apps don&#x27;t properly clean up their URL handlers when uninstalled.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147791&amp;goto=item%3Fid%3D27145911%2327147791">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27147209'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147209' onclick='return vote(event, this, "up")' href='vote?id=27147209&amp;how=up&amp;auth=33b90f34027016fe80c0b99b0621a277daa59bff&amp;goto=item%3Fid%3D27145911#27147209'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=jedberg" class="hnuser">jedberg</a> <span class="age"><a href="item?id=27147209">2 days ago</a></span> <span id="unv_27147209"></span><span class="par"></span> <a class="togg" n="3" href="javascript:void(0)" onclick="return toggle(event, 27147209)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">It seems that it&#x27;s not very effective in Linux.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147209&amp;goto=item%3Fid%3D27145911%2327147209">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                    <tr class='athing comtr' id='27147380'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27147380' onclick='return vote(event, this, "up")' href='vote?id=27147380&amp;how=up&amp;auth=a803a09d027d5c888a049358ed4ec7ea350daf6d&amp;goto=item%3Fid%3D27145911#27147380'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=valve1" class="hnuser">valve1</a> <span class="age"><a href="item?id=27147380">2 days ago</a></span> <span id="unv_27147380"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147380)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Yeah, we tested it on MacOS Big Sur mostly. Nobody on the team had linux so we didn&#x27;t really test there.
    It can be made to work with better timings for the measurements etc.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147380&amp;goto=item%3Fid%3D27145911%2327147380">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                <tr class='athing comtr' id='27164559'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="80"></td><td valign="top" class="votelinks">
          <center><a id='up_27164559' onclick='return vote(event, this, "up")' href='vote?id=27164559&amp;how=up&amp;auth=7e9e220efd1a81fbf11506fcd2b701c6dedf3515&amp;goto=item%3Fid%3D27145911#27164559'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=samjanis" class="hnuser">samjanis</a> <span class="age"><a href="item?id=27164559">1 day ago</a></span> <span id="unv_27164559"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27164559)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">Linux Firefox
    used Schemeflood!
                    v<p>It&#x27;s not very
    effective....</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27164559&amp;goto=item%3Fid%3D27145911%2327164559">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                      <tr class='athing comtr' id='27147355'><td>
                    <table border='0'>  <tr>    <td class='ind'><img src="s.gif" height="1" width="40"></td><td valign="top" class="votelinks">
          <center><a id='up_27147355' onclick='return vote(event, this, "up")' href='vote?id=27147355&amp;how=up&amp;auth=8bf57c33ed70b0f929bc56e7d9fcadcd5a340ce2&amp;goto=item%3Fid%3D27145911#27147355'><div class='votearrow' title='upvote'></div></a></center>    </td><td class="default"><div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
              <a href="user?id=Guest81" class="hnuser">Guest81</a> <span class="age"><a href="item?id=27147355">2 days ago</a></span> <span id="unv_27147355"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27147355)">[–]</a>          <span class='storyon'></span>
                      </span></div><br><div class="comment">
                      <span class="commtext c00">worked for me on firefox and tor.</span>
                  <div class='reply'>        <p><font size="1">
                          <u><a href="reply?id=27147355&amp;goto=item%3Fid%3D27145911%2327147355">reply</a></u>
                      </font>
          </div></div></td></tr>
            </table></td></tr>
                          </table>
          <br><br>
      </td></tr>
    <tr><td><img src="s.gif" height="10" width="0"><table width="100%" cellspacing="0" cellpadding="1"><tr><td bgcolor="#ff6600"></td></tr></table><br><center><span class="yclinks"><a href="newsguidelines.html">Guidelines</a>
            | <a href="newsfaq.html">FAQ</a>
            | <a href="lists">Lists</a>
            | <a href="https://github.com/HackerNews/API">API</a>
            | <a href="security.html">Security</a>
            | <a href="http://www.ycombinator.com/legal/">Legal</a>
            | <a href="http://www.ycombinator.com/apply/">Apply to YC</a>
            | <a href="mailto:hn@ycombinator.com">Contact</a></span><br><br><form method="get" action="//hn.algolia.com/">Search:
              <input type="text" name="q" value="" size="17" autocorrect="off" spellcheck="false" autocapitalize="off" autocomplete="false"></form>
                </center></td></tr>
          </table></center></body><script type='text/javascript' src='hn.js?t6cz9VDBRNMVBhMCuz1j'></script></html>
    "###)
}