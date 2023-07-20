use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn add_lines() -> Result<(), Box<dyn std::error::Error>> {
    let p1 = r#"#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
    int w = 0;
}
"#;

    let p2 = r#"#include <iostream>

using namespace std;

void main()
{
cout << "hello! \" world!";
int r = 10;
for(int i =0; i<10)
{
    moo mooo
}
int w = 0;
}
"#;

    let result = r#"#include <iostream>
using namespace std;
void main()
{
cout << "hello! \" world!";
+ int r = 10;
+ for(int i =0; i<10)
+ {
+ moo mooo
+ }
int w = 0;
}"#;

    let file1 = assert_fs::NamedTempFile::new("file1.cpp")?;
    file1.write_str(p1)?;

    let file2 = assert_fs::NamedTempFile::new("file2.cpp")?;
    file2.write_str(p2)?;

    let mut cmd = Command::cargo_bin("diff")?;
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(result));

    Ok(())
}