from pathlib import Path
import shutil
import re

# ルート
ROOT = Path("/Users/komotokenta/Docker/kyopro")
ATCODER_DIRS = [
    ROOT / "abc_training",
    ROOT / "arc_training",
    ROOT / "agc_training",
    ROOT / "typical90",
    ROOT / "Others",
]
TARGET_DIR = ROOT / "atcoder_training"

def traverse_file():
    """atcoderの提出ファイルを列挙する
    """
    for contest in ATCODER_DIRS:
        for filepath in contest.glob("**/*.*"):
            # ファイルのみを列挙
            if filepath.is_file():
                yield filepath


def get_file_url(filepath: Path) -> str:
    """ファイルのパスから問題URLを取得する
    """
    url = ""

    # ファイル名のURLを取得
    with open(filepath, "r") as f:
        try:
            contents = f.readlines()
        except:
            return
        if len(contents) >= 4:
            line4 = str(contents[3])  # urlがある行
            matched = re.search(r"http.*$", line4)  # urlにマッチさせる
            if matched:
                url = matched[0]

    return url


def parse_url(url: str) -> dict:
    """URLをもとに、必要な情報を取得する
    """
    if "atcoder" in url:
        words = url.split("/")
        contest = words[words.index("contests") + 1]
        return {
            "site": "atcoder",
            "contest": contest,
        }


def main():
    """ファイルのコピーを行う
    """
    for filepath in traverse_file():
        # 問題のURLを取得
        file_url = get_file_url(filepath)
        if file_url is None:
            continue

        # URLをパースする
        info = parse_url(file_url)
        if info is None:
            continue

        # ディレクトリを作成する
        dir = TARGET_DIR / info["contest"]
        dir.mkdir(exist_ok=True)

        # ファイルをコピーする
        shutil.copy2(filepath.resolve(), dir.resolve())
        

if __name__ == "__main__":
    main()
