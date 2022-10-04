"""
# 問題を解くファイルの作成
1. url、言語を入力
2. 問題のページを取得し、分類 -> ./scrape.py で行う
3. テンプレートをもとにファイルを作成
"""

import sys
import re
import scrape
from pathlib import Path

problem_dir = {
    "abc": "abc_training",
    "arc": "arc_training",
    "agc": "agc_training",
    "typical90": "typical90",
    "other": "Others/others",
    "aoj": "AOJ",
}

def make_filename(data: dict, lang: str="py") -> Path:
    """ファイル名を作成する"""

    # ディレクトリ名
    dir = Path(__file__).parent.parent
    
    if data["service"] == "atcoder":
        # コンテストで分類
        dir /= problem_dir[data["contest_type"]]

        # 問題難易度で分類
        if data["contest_type"] in ("abc", "arc", "agc"):
            dir /= data["problem_type"].upper()

    if data["service"] == "aoj":
        dir /= problem_dir["aoj"]

    # ファイル名
    filename = re.sub(
        r"-|:|/|,|\.|\(|\)|!|'",
        "",
        data["title"]
    )

    filename = re.sub(
        r" +",
        "_",
        filename
    )

    # typical90に対応
    filename = re.sub(
        r"（.+）",
        "",
        filename
    )

    filename += "." + lang

    return dir / filename


def make_header(data: dict, lang: str):
    """C++ / Python用のヘッダーを作成"""

    comment = {"py": "#", "cpp": "//", "hs": "--", "rs": "//"}
    line = "-" * 40

    template = [
        data["title"].center(40),
        line,
        "問題",
        data["url"],
        line,
    ]

    result = "\n".join(
        map(
            lambda x: f"{comment[lang]} {x}",
            template
        )
    )

    return result


def main():
    _, url, lang, *_ = sys.argv + ["cpp"]
    
    lang = {
        "py": "py",
        "python": "py",
        "cpp": "cpp",
        "c++": "cpp",
        "hs": "hs",
        "haskell": "hs",
        "rust": "rs",
        "rs": "rs",
    }[lang]  # デフォルトは"py"

    data = scrape.get_problem_info(url)
    filename = make_filename(data, lang)
    header = make_header(data, lang)

    # ディレクトリの作成
    filename.parent.mkdir(exist_ok=True)

    # ファイルの作成
    try:
        filename.touch(exist_ok=False)

        # ヘッダーの埋め込み
        with open(filename, "w") as f:
            f.write(header)
    except:
        print("This file already exists.")
    
    print(">", filename)


if __name__ == "__main__":
    main()
