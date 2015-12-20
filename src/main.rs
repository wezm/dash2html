extern crate rusqlite;
extern crate regex;

use regex::Regex;
use rusqlite::SqliteConnection;

// Dash Snippets Schema, as at Dash 2.2.6
// CREATE TABLE tagsIndex(tid INTEGER, sid INTEGER);
// CREATE TABLE snippets(sid INTEGER PRIMARY KEY, title TEXT, body TEXT, syntax VARCHAR(20), usageCount INTEGER, FOREIGN KEY(sid) REFERENCES tagsIndex(sid) ON DELETE CASCADE ON UPDATE CASCADE);
// CREATE TABLE tags(tid INTEGER PRIMARY KEY, tag TEXT UNIQUE, FOREIGN KEY(tid) REFERENCES tagsIndex(tid) ON DELETE CASCADE ON UPDATE CASCADE);
// CREATE TABLE smartTags(stid INTEGER PRIMARY KEY, name TEXT, query TEXT);

#[derive(Debug)]
struct Snippet {
    id: i32,
    title: String,
    body: String,
    syntax: String,
}

struct Tag {
    id: i32,
    tag: String,
}

// From Rails { '&' => '&amp;', '>' => '&gt;', '<' => '&lt;', '"' => '&quot;', "'" => '&#x27;' }
fn escape(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace(">", "&gt;")
        .replace(">", "&lt;")
        .replace("\"", "&quot;")
        .replace("'", "&#27;")
}

fn main() {
    let variable_re = Regex::new(r"__.*?__").unwrap();
    let placeholder_re = Regex::new(r"(?i)@(?:time|clipboard|cursor|date)").unwrap();
    let conn = SqliteConnection::open("/Users/wmoore/Dropbox (Personal)/Dash Snippets.dash").unwrap();

    let mut stmt = conn.prepare("SELECT sid, title, body, syntax FROM snippets ORDER BY title").unwrap();
    let mut tags_stmt = conn.prepare("SELECT tags.tid, tags.tag FROM tags, tagsIndex WHERE tags.tid = tagsIndex.tid AND tagsIndex.sid = ?").unwrap();

    let snippet_iter = stmt.query_map(&[], |row| {
        Snippet {
            id: row.get(0),
            title: row.get(1),
            body: row.get(2),
            syntax: row.get(3),
        }
    }).unwrap();

    let head = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Dash Snippets</title>
  <meta name="description" content="">
  <meta name="generator" content="dash2html" />
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <link href="//fonts.googleapis.com/css?family=Raleway:400,300,600" rel="stylesheet" type="text/css">
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/skeleton/2.0.4/skeleton.min.css">
  <style>
    .snippet-body {
      white-space: pre-wrap;
    }
    .tag {
      background-color: aliceblue;
      padding: 2px 8px;
      border-radius: 12px;
      border: 1px solid lightblue;
      font-size: smaller;
    }
    .variable {
      color: mediumvioletred;
    }
    .placeholder {
      color: blueviolet;
    }
    footer {
      font-size: small;
      color: lightslategray;
      text-align: center;
    }
  </style>
</head>
<body>

  <!-- Primary Page Layout
  –––––––––––––––––––––––––––––––––––––––––––––––––– -->
  <div class="container">
    <header class="row">
      <h1>Dash Snippets</h1>
    </header>
    <section class="row">
      <table>
        <thead>
          <tr><th>Abbreviation</th><th>Snippet</th><th>Tags</th></tr>
        </thead>
        <tbody>
"#;

    let footer = r#"
        </tbody>
      </table>
    </section>
    <footer>
      Generated using
      <a href="https://github.com/wezm/dash2html">dash2html</a>
    </footer>
  </div>

<!-- End Document
  –––––––––––––––––––––––––––––––––––––––––––––––––– -->
</body>
</html>
"#;
    println!("{}", head);

    for person in snippet_iter {
        let snippet = person.unwrap();

        let tags_iter = tags_stmt.query_map(&[&snippet.id], |row| {
            Tag {
                id: row.get(0),
                tag: row.get(1),
            }
        }).unwrap();

        let tags: Vec<Tag> = tags_iter.map(|tag| tag.unwrap()).collect();

        // Skip snippets tagged with private
        if !tags.iter().any(|tag| tag.tag == "public") {
            continue;
        }

        // Generate tags markup, skip the public one that's on all of them
        let html_tags: Vec<String> = tags.iter()
            .filter_map(|tag| if tag.tag != "public" { Some(format!(r#"<span class="tag tag-{}">{}</span>"#, tag.id, escape(&tag.tag))) } else { None })
            .collect();

        let snippet_body = placeholder_re.replace_all(&variable_re.replace_all(&escape(&snippet.body), r#"<span class="variable">$0</span>"#), r#"<span class="placeholder">$0</span>"#);

        println!(r#"<tr id="snippet-{}"><td><code>{}</code></td><td class="snippet-body">{}</td><td>{}</td></tr>"#, snippet.id, escape(&snippet.title), &snippet_body, html_tags.join(" "));
    }

    println!("{}", footer);
}

