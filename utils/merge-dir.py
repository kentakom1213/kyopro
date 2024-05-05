"""
`atcoder_training`ディレクトリを`contests`ディレクトリにマージする
"""

import argparse
from pathlib import Path
import shutil
from pprint import pprint


def merge_dir(root_dir: Path, dir_a: str, dir_b: str, save_dir: str = "", ignore: list[str] = []):
    """
    `root_dir/dir_a`と`root_dir/dir_b`を再帰的にマージする
    """
    # マージするパスの初期値
    if not save_dir:
        save_dir = f"merge_{dir_a}_{dir_b}"

    save_dir_path = root_dir / save_dir

    try:
        save_dir_path.mkdir()
    except Exception:
        raise ValueError(f"指定されたディレクトリ {save_dir} はすでに存在します．")
    
    # 再帰的にマージ
    _merge_recur(
        root_dir / dir_a,
        root_dir / dir_b,
        save_dir_path,
        ignore
    )


def _merge_recur(dir_a: Path, dir_b: Path, save_dir: Path, ignore: list[str]):
    # ファイルの取り出し
    Fa = sorted(dir_a.iterdir())
    Fb = sorted(dir_b.iterdir())

    # インデックスなど
    Na, Nb = len(Fa), len(Fb)
    i = j = 0

    # マージ
    while i < Na or j < Nb:
        if i < Na and j < Nb and Fa[i].name == Fb[j].name:
            # 無視する場合
            if Fa[i].name in ignore:
                pass
            # 同じ値を取る場合
            elif Fa[i].is_dir():
                save_dir_nxt = save_dir / Fa[i].name
                save_dir_nxt.mkdir()
                _merge_recur(Fa[i], Fb[j], save_dir_nxt, ignore)
            # ファイルを保存
            else:
                file_name = Fa[i].name
                _merge_file(Fa[i], Fb[j], save_dir / file_name)

            # インクリメント
            i += 1
            j += 1

        elif j >= Nb or (i < Na and Fa[i].name < Fb[j].name):
            # 無視する場合
            if Fa[i].name in ignore:
                pass
            # aを採用する場合
            elif Fa[i].is_dir():
                save_dir_nxt = save_dir / Fa[i].name
                save_dir_nxt.mkdir(exist_ok=True)
                shutil.copytree(Fa[i], save_dir_nxt, dirs_exist_ok=True, ignore=shutil.ignore_patterns(*ignore))
            else:
                shutil.copy(Fa[i], save_dir)

            # インクリメント
            i += 1

        else:
            # 無視する場合
            if Fb[j].name in ignore:
                pass
            # bを採用する場合
            elif Fb[j].is_dir():
                save_dir_nxt = save_dir / Fb[j].name
                save_dir_nxt.mkdir(exist_ok=True)
                shutil.copytree(Fb[j], save_dir_nxt, dirs_exist_ok=True, ignore=shutil.ignore_patterns(*ignore))
            else:
                shutil.copy(Fb[j], save_dir)

            # インクリメント
            j += 1


def _merge_file(file_a: Path, file_b: Path, save_file: Path):
    """同じ名前のファイルをマージする

    Args:
        file_a (Path): 1つめのファイル
        file_b (Path): 2つめのファイル
        save_file (Path): 保存するファイル
    """
    # 拡張子を追加
    save_file = save_file.parent / (save_file.name + ".merge")

    print(f"[Merge] {save_file}")

    source_a = file_a.read_text()
    source_b = file_b.read_text()

    # 文字列を結合
    merged = "\n".join((
        source_a,
        f"===== ↑ {file_a} | ↓ {file_b} =====",
        source_b
    ))

    # 書き込み
    save_file.write_text(merged)


if __name__ == "__main__":
    
    ROOT = Path(__file__).parent

    # 引数
    parser = argparse.ArgumentParser()

    parser.add_argument("dir_a")
    parser.add_argument("dir_b")
    parser.add_argument("-s", "--save-dir", default="")
    parser.add_argument("-i", "--ignore", nargs="*", default=["target", ".vscode", "testcases"])
    
    # パース
    args = parser.parse_args()

    # マージ
    merge_dir(ROOT, args.dir_a, args.dir_b, args.save_dir, args.ignore)
