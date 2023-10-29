"""
# テスト
scrape.py, make_file.pyのテストコード
"""


import scrape, make_file

atcoder_url = [
    "https://atcoder.jp/contests/abc136/tasks/abc136_a",
    "https://atcoder.jp/contests/abc014/tasks/abc014_4",
    "https://atcoder.jp/contests/arc134/tasks/arc134_b",
    "https://atcoder.jp/contests/arc099/tasks/arc099_a",
    "https://atcoder.jp/contests/agc026/tasks/agc026_b",
    "https://atcoder.jp/contests/agc004/tasks/agc004_e",
    "https://atcoder.jp/contests/tokiomarine2020/tasks/tokiomarine2020_a",
    "https://atcoder.jp/contests/aising2020/tasks/aising2020_a",
    "https://atcoder.jp/contests/typical90/tasks/typical90_aw",
]

aoj_url = [
    "https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A",
    "https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0537",
    "https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B&lang=jp",
    "https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0202",
    "https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=3246",
    "https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2377",
    "https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0306",
]

def test_scrape():
    print("test \"scrape.py\"")

    for url in atcoder_url:
        res = scrape.get_problem_info(url)
        print(res)
    
    for url in aoj_url:
        res = scrape.get_problem_info(url)
        print(res)

    print("OK\n")
    

def test_make_filename():
    print("test \"make_file.py\" - make_filename")

    for url in atcoder_url:
        data = scrape.get_problem_info(url)
        res = make_file.make_filename(data)
        print(res)
    
    for url in aoj_url:
        data = scrape.get_problem_info(url)
        res = make_file.make_filename(data)
        print(res)

    print("OK\n")


def test_make_header():
    print("test \"make_file.py\" - make_header")

    urls = [
        atcoder_url[0],
        atcoder_url[2],
        aoj_url[3],
        aoj_url[4]
    ]

    datas = list(map(scrape.get_problem_info, urls))
    headers_py = map(lambda d: make_file.make_header(d, "py"), datas)
    headers_cpp = map(lambda d: make_file.make_header(d, "cpp"), datas)

    print("Template for Python")
    print(*headers_py, sep="\n\n")

    print("\nTemplate for C++")
    print(*headers_cpp, sep="\n\n")

    print("OK\n")


if __name__ == "__main__":
    test_scrape()
    test_make_filename()
    test_make_header()
