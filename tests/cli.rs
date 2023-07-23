use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn comparison_add_function_and_update_line() -> Result<(), Box<dyn std::error::Error>> {
    let p1 = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
    int c = 145;
}"#;

    let p2: &str = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hellfo! \" world!";
    int c = 145;
    int g = 444;
}

int func(int a)
{
    int b = a;
    return b + 5;
}"#;

    let result:&str = r#"(#include <iostream>
using namespace std;
void main()
{
- cout << "hello! \" world!";
+ cout << "hellfo! \" world!";
int c = 145;
+ int g = 444;
}
+ int func(int a)
+ {
+ int b = a;
+ return b + 5;
+ }"#;

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

#[test]
fn remove_single_line() -> Result<(), Box<dyn std::error::Error>> {
    let p1 = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
    int c = 145;
}"#;

let p2: &str = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
}"#;

let result: &str = r#"(#include <iostream>
using namespace std;
void main()
{
cout << "hello! \" world!";
- int c = 145;
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

#[test]
fn comparison_remove_single_function() -> Result<(), Box<dyn std::error::Error>> {
    let p1 = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
}

int func(int a)
{
    int b = a;
    return b + 5;
}"#;

let p2: &str = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
}"#;

let result: &str = r#"(#include <iostream>
using namespace std;
void main()
{
cout << "hello! \" world!";
}
- int func(int a)
- {
- int b = a;
- return b + 5;
- }"#;

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

#[test]
fn comparison_insert_several_lines() -> Result<(), Box<dyn std::error::Error>> {
    let p1 = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
    cout << "monkey monkey";
    int a = 10;
}"#;

let p2: &str = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
    int b = 56;
    cout << "bananas";
    cout << "monkey monkey";
    int a = 10;
}"#;

let result: &str = r#"(#include <iostream>
using namespace std;
void main()
{
cout << "hello! \" world!";
+ int b = 56;
+ cout << "bananas";
cout << "monkey monkey";
int a = 10;
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

#[test]
fn comparison_add_for_loop() -> Result<(), Box<dyn std::error::Error>> {
    let p1 = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
    int w = 0;
    int c = 145;
}"#;

let p2: &str = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
    int w = 0;
    for(int a = 10; a < 100; ++a)
    {
        w = w * 2;
        cout << "weeeeee!";
    }
    int c = 145;
}"#;

let result: &str = r#"(#include <iostream>
using namespace std;
void main()
{
cout << "hello! \" world!";
int w = 0;
+ for(int a = 10; a < 100; ++a)
+ {
+ w = w * 2;
+ cout << "weeeeee!";
+ }
int c = 145;
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

#[test]
fn comparison_remove_for_loop() -> Result<(), Box<dyn std::error::Error>> {
    let p1 = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
    int w = 0;
    for(int a = 10; a < 100; ++a)
    {
        w = w * 2;
        cout << "weeeeee!";
    }
    int c = 145;
}"#;

let p2: &str = r#"(#include <iostream>

using namespace std;

void main()
{
    cout << "hello! \" world!";
    int w = 0;
    int c = 145;
}"#;

let result: &str = r#"(#include <iostream>
using namespace std;
void main()
{
cout << "hello! \" world!";
int w = 0;
- for(int a = 10; a < 100; ++a)
- {
- w = w * 2;
- cout << "weeeeee!";
- }
int c = 145;
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
