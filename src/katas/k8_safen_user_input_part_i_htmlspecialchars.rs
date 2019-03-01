#![allow(unused)]

/*
    Kata (8kyu): Safen User Input Part I - htmlspecialchars
    Url: https://www.codewars.com/kata/safen-user-input-part-i-htmlspecialchars/train/rust

    Safen User Input Part I - htmlspecialchars

    You are a(n) novice/average/experienced/professional/world-famous
    Web Developer (choose one) who owns a(n) simple/clean/slick/beautiful/
    complicated/professional/business website (choose one or more)
    which contains form fields so visitors can send emails or leave a comment
    on your website with ease. However, with ease comes danger.
    Every now and then, a hacker visits your website and attempts
    to compromise it through the use of XSS (Cross Site Scripting).
    This is done by injecting script tags into the website through form fields
    which may contain malicious code (e.g. a redirection to a malicious website
    that steals personal information).

    Mission
    Your mission is to implement a function htmlspecialchars()
    that converts the following potentially harmful characters:

    < --> &lt;
    > --> &gt;
    " --> &quot;
    & --> &amp;
    Good luck :D

    Extension
    If you are an experienced Javascript programmer, try shortening your code
    as much as possible and optimise it to minimise run time.
    Experienced programmers should be able to complete this exercise
    in one line of code.
*/

fn html_special_chars(html: &str) -> String {
    html.chars()
        .map(|c| match c {
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '"' => "&quot;".to_owned(),
            '&' => "&amp;".to_owned(),
            _ => c.to_string(),
        })
        .collect()
}

#[test]
fn sample_tests() {
    assert_eq!(
        html_special_chars("<h2>Hello World</h2>"),
        "&lt;h2&gt;Hello World&lt;/h2&gt;"
    );
    assert_eq!(
        html_special_chars("Hello, how would you & I fare?"),
        "Hello, how would you &amp; I fare?"
    );
    assert_eq!(
        html_special_chars("How was \"The Matrix\"?  Did you like it?"),
        "How was &quot;The Matrix&quot;?  Did you like it?"
    );
    assert_eq!(
        html_special_chars("<script>alert('Website Hacked!');</script>"),
        "&lt;script&gt;alert('Website Hacked!');&lt;/script&gt;"
    );
}
