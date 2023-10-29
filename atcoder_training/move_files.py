from pathlib import Path
import re

# ルート
ROOT = Path("/Users/komotokenta/Docker/kyopro")
ATCODER_DIRS = [
    ROOT / "abc_training",
    ROOT / "arc_training",
    ROOT / "agc_training",
    ROOT / "typical90",
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


def get_file_data(filepath: Path) -> dict:
    url = ""
    with open(filepath, "r") as f:
        contents = f.readlines()
        if len(contents) >= 4:
            line4 = str(contents[3])  # urlがある行
            matched = re.search(r"http.*$", line4)  # urlにマッチさせる
            if matched:
                url = matched[0]

    print(url)


def main():
    """ファイルのコピーを行う
    """
    for file in traverse_file():
        print(get_file_data(file))


if __name__ == "__main__":
    main()
