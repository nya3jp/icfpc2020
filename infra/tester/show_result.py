import argparse
import logging
import os
import subprocess
import sys
import threading
import urllib.request
import pprint


def main():
    # attacker 0
    # defender 1
    ret = demodulate(
        iter(post_to_server(modulate((5, (int(sys.argv[1]), None))))))
    result = to_list_ish(ret)
    pprint.pprint(result)

    attacker_alive = False
    defender_alive = False
    for machine in result[5][1][-1][1][0][:-1]:
        team, alive = is_alive(machine)
        if team == 0:
            attacker_alive = attacker_alive or alive
        else:
            defeneder_alive = defender_alive or alive
    for machine in result[5][1][-1][1][1][:-1]:
        team, alive = is_alive(machine)
        if team == 0:
            attacker_alive = attacker_alive or alive
        else:
            defeneder_alive = defender_alive or alive
    if attacker_alive and not defender_alive:
        print('attacker wins')
    else:
        print('defender wins')


def is_alive(machine):
    pprint.pprint(machine)
    team = machine[0]
    alive = machine[4][3]
    return (team, alive != 0)


def post_to_server(command):
    req = urllib.request.Request(
        url='https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=' +
        os.environ['ICFPC_API_KEY'],
        data=command.encode('utf-8'),
        method='POST')
    req.add_header('Content-Type', 'text/plain')
    with urllib.request.urlopen(req) as resp:
        if resp.status != 200:
            logging.error('tester: non-200 response %s', resp)
            sys.exit(1)
        return resp.read().decode('utf-8').strip()


def to_list_ish(v):
    if v is None:
        return []
    if type(v) is int:
        return v
    first = to_list_ish(v[0])
    second = to_list_ish(v[1])
    if type(second) is list:
        return [first] + second
    return (first, second)


def modulate(v):
    """Modulates a value

    >>> modulate((1, (81740, None)))
    '110110000111011111100001001111110100110000'
    >>> modulate(0)
    '010'
    >>> modulate(1)
    '01100001'
    >>> modulate(-1)
    '10100001'
    >>> modulate(81740)
    '0111111000010011111101001100'
    """
    if v is None:
        return "00"
    if type(v) is tuple:
        if len(v) != 2:
            raise ValueError()
        return "11" + modulate(v[0]) + modulate(v[1])
    ret = ""
    if v >= 0:
        ret += "01"
    else:
        ret += "10"
        v *= -1

    bits = ""
    while v:
        bits += str(v % 2)
        v //= 2
    bits = bits[::-1]
    bitlen = 0
    while bitlen * 4 < len(bits):
        bitlen += 1
    ret += '1' * bitlen + '0'
    ret += '0' * (bitlen * 4 - len(bits)) + bits
    return ret


def demodulate(it):
    """Demodulates a value

    >>> demodulate(iter("110110000111011111100001001111110100110000"))
    (1, (81740, None))
    >>> demodulate(iter("010"))
    0
    >>> demodulate(iter("01100001"))
    1
    >>> demodulate(iter("10100001"))
    -1
    """
    t0 = next(it)
    t1 = next(it)
    if t0 == '0' and t1 == '0':
        return None
    if t0 == '1' and t1 == '1':
        first = demodulate(it)
        second = demodulate(it)
        return (first, second)
    bits = 0
    while next(it) == '1':
        bits += 4
    v = 0
    for i in reversed(range(bits)):
        if next(it) == '1':
            v = v + (1 << i)
    return v if t1 == '1' else -v


if __name__ == "__main__":
    main()
