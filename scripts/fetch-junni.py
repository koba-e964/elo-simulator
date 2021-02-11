#!/usr/bin/env python3
"""
Fetches junni-sen's information from www.shogi.or.jp.
Usage: ./scripts/fetch-junni.py YEAR/CLASS >output.json
e.g.: ./scripts/fetch-junni.py 79b1 >79b1.json
"""

import sys
import requests
import re
from bs4 import BeautifulSoup
import json

URL = 'https://www.shogi.or.jp/match/junni/2020/{}/index.html'

result_mapping = {'○': 'won', '●': 'lost',
                  '先': 'plays first', '': 'plays second'}


def main(year_class):
    response = requests.get(URL.format(year_class))
    response.encoding = 'Shift_JIS'
    soup = BeautifulSoup(response.text, 'html.parser')

    mappings = []

    for tr in soup.find_all('tr'):
        tds = tr.find_all('td')
        if len(tds) >= 2 and tds[1].text.isdigit():
            up_down = tds[0].text
            rank = int(tds[1].text)
            name = tds[2].text
            wins = int(tds[3].text)
            losses = int(tds[4].text)
            results = []
            for i in range(5, len(tds)):
                res_opponent = tds[i].text.split("\n")
                if len(res_opponent) == 2:
                    res, opponent = res_opponent
                    opponent = opponent.strip()
                    ascii_result = result_mapping[res] if (
                        res in result_mapping) else res
                    results.append(
                        {'game_index': i - 5, 'result': ascii_result, 'opponent': opponent})
            mappings.append(
                {'initial_rank': rank, 'name': name, 'wins': wins, 'losses': losses, 'up_down': up_down, 'results': results})

    print(json.dumps({'data': mappings}, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    if len(sys.argv) != 2:
        msg = """
Usage: ./scripts/fetch-junni.py YEAR/CLASS >output.json
e.g.: ./scripts/fetch-junni.py 79b1 >79b1.json
"""
        print(msg, file=sys.stderr)
        exit(1)
    year_class = sys.argv[1]
    main(year_class)
