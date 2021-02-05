#!/usr/bin/env python3
"""
Fetches all players' information from www.shogi.or.jp.
Usage: ./scripts/fetch-players.py >output.json
"""

import requests
import re
from bs4 import BeautifulSoup
import json

URL = 'https://www.shogi.or.jp/player/'


def main():
    response = requests.get(URL)
    response.encoding = 'UTF-8'
    soup = BeautifulSoup(response.text, 'html.parser')
    url_pattern = '/player/pro/([0-9]*).html'

    mappings = []

    for p in soup.find_all('p'):
        if 'class' in p.attrs and p['class'] == ['ttl']:
            if p.a is not None:
                addr = p.a['href']
                name = p.a.text
                m = re.search(url_pattern, addr)
                index = int(m.group(1))
                mappings.append({'id': index, 'name': name})

    print(json.dumps({'data': mappings}, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
