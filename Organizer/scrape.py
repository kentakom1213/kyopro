"""
# ページのスクレイピング
1. webページの種類を判別
2. サービスごとに解析

## 取得する情報
### AtCoder
- 問題タイトル
- url
- コンテストの種類
- コンテスト名 / 問題番号
- 点数

### AOJ
- 問題タイトル
- url
"""

from string import ascii_lowercase
import re
import requests
from bs4 import BeautifulSoup


def get_problem_info(url: str):
    """問題の情報を取得する"""

    parser = {
        "atcoder": scrape_atcoder,
        "aoj"    : scrape_aoj,
    }

    # 適切な関数を呼び出す
    service = classify_service(url)
    result = parser[service](url)

    return result


def classify_service(url: str):
    """Webサービスを分類する"""

    if "atcoder.jp" in url:
        return "atcoder"
    elif "judge.u-aizu.ac.jp" in url:
        return "aoj"
    else:
        raise ValueError(f"対応していないサイトです。\n> {url}")


def scrape_atcoder(url: str):
    """AtCoderサイトのスクレイピングを行う"""

    # ページの取得
    page = requests.get(url)
    soup = BeautifulSoup(page.text, "html.parser")

    # urlの解析
    tail = url.split("/")[-1]

    contest_type = tail[:3]
    if contest_type not in ("abc", "arc", "agc"):
        contest_type = "other"

    problem_type = tail.split("_")[-1]
    if re.match(r"\d+", problem_type):
        problem_type = ascii_lowercase[int(problem_type) - 1]


    # 点数の解析
    point_tag = soup.find("var").text
    point = point_tag if re.match(r"\d+", point_tag) else "-"

    # 出力
    result = {
        "service": "atcoder",
        "title": soup.title.text,
        "url": url,
        "contest_type": contest_type,
        "problem_type": problem_type,
        "point": point,
    }

    return result


def scrape_aoj(url: str):
    """AOJサイトのスクレイピングを行う"""

    # ページの取得
    page = requests.get(url)
    soup = BeautifulSoup(page.text, "html.parser")

    # タイトルを見つける
    h1 = soup.find_all("h1", class_="")
    h2 = soup.find_all("h2")
    h3 = soup.find_all("h3")

    if h1:
        title = h1[0].text
    elif h2:
        title = h2[0].text
    else:
        title = h3[0].text
    
    title = " ".join(title.split())

    result = {
        "service": "aoj",
        "title": title,
        "url": url,
    }

    return result
