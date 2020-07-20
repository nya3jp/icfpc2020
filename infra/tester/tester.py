import argparse
import logging
import os
import subprocess
import sys
import threading
import urllib.request
import pprint

logging.basicConfig(level=logging.INFO)

tutorials = [
    "1101100001110110000100",
    "1101100001110110001000",
    "1101100001110110001100",
    "1101100001110110010000",
    "1101100001110110010100",
    "1101100001110110011000",
    "1101100001110110011100",
    "1101100001110110100000",
    "1101100001110110100100",
    "1101100001110110101000",
    "1101100001110110101100",
    "1101100001110110110000",
    "1101100001110110110100",
]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('subcommand',
                        help="subcommand: 'tutorial' or 'battle'")
    parser.add_argument(
        'arg1',
        help="if tutorial, the tutorial number. if battle, a binary path")
    parser.add_argument('arg2', help="a binary path")
    parser.add_argument('--logfile',
                        '--logfile1',
                        dest='logfile1',
                        help='log file for the first bot')
    parser.add_argument('--logfile2', help='log file for the second bot')
    parser.add_argument('--logprefix',
                        '--logprefix1',
                        dest='logprefix1',
                        help='log prefix for the first bot')
    parser.add_argument('--logprefix2', help='log prefix for the second bot')

    args = parser.parse_args()

    if args.subcommand == 'tutorial':
        outfile = sys.stderr
        if args.logfile1:
            outfile = open(args.logfile1, mode='a')
            print("============", file=outfile)
        outprefix = args.logprefix1

        pk = get_tutorial_playerkey(int(args.arg1))
        print("tester: PlayerKey %d" % (pk, ))
        run_bot(args.arg2, pk, True, outprefix, outfile)
        show_result(pk)
        return
    elif args.subcommand == 'battle':
        outfile1 = sys.stderr
        if args.logfile1:
            outfile1 = open(args.logfile1, mode='a')
            print("============", file=outfile1)
        outprefix1 = 'bot1'
        if args.logprefix1:
            outprefix1 = args.logprefix1
        outfile2 = sys.stderr
        if args.logfile2:
            outfile2 = open(args.logfile2, mode='a')
            print("============", file=outfile2)
        outprefix2 = 'bot2'
        if args.logprefix2:
            outprefix2 = args.logprefix2

        pks = get_random_playerkeys()
        print("tester: PlayerKey %d %d" % pks)
        t = threading.Thread(target=run_bot,
                             args=(args.arg1, pks[0], False, outprefix1,
                                   outfile1))
        t.start()
        run_bot(args.arg2, pks[1], False, outprefix2, outfile2)
        t.join()
        show_result(pks[0])
        return

    print("tester.py tutorial NUM BINARY_PATH")
    print("tester.py battle BINARY_PATH BINARY_PATH")
    sys.exit(1)


def show_result(player_key):
    # attacker 0
    # defender 1
    ret = demodulate(iter(post_to_server(modulate((5, (player_key, None))))))
    result = to_list_ish(ret)
    pprint.pprint(result)
    if result[3] < 0:
        print('the game ended with an error. check the requests from the bots')
        return

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
        print('attacker (first bot) wins')
    else:
        print('defender (second bot) wins')
    print('replay: https://icfpcontest2020.github.io/#/visualize?playerkey=%d' % player_key)


def is_alive(machine):
    pprint.pprint(machine)
    team = machine[0]
    alive = machine[4][3]
    return (team, alive != 0)


def wrap_err(err_pipe, prefix, outfile):
    buf = ''
    for token in err_pipe:
        buf += token
        if '\n' in buf:
            if prefix:
                buf = "[%s] %s" % (prefix, buf.rstrip())
            print(buf, file=outfile)
            buf = ''


def run_bot(main_program, player_key, is_tutorial, prefix, outfile):
    modified_env = os.environ.copy()
    if is_tutorial:
        modified_env["TUTORIAL_MODE"] = "1"
    p = subprocess.Popen([main_program, str(player_key)],
                         bufsize=0,
                         stdin=subprocess.PIPE,
                         stdout=subprocess.PIPE,
                         stderr=subprocess.PIPE,
                         env=modified_env,
                         close_fds=True,
                         encoding='utf-8')
    t = threading.Thread(target=wrap_err, args=(p.stderr, prefix, outfile))
    t.start()
    (child_stdin, child_stdout) = (p.stdin, p.stdout)
    while True:
        line = child_stdout.readline().strip()
        if line == '':
            return
        logging.info('tester: send: %s', line)
        body = post_to_server(line)
        logging.info('tester: recv: %s', body)
        child_stdin.write(body + '\n')
    t.join()


def get_tutorial_playerkey(tutorial_num):
    command = tutorials[tutorial_num - 1]
    body = demodulate(iter(post_to_server(command)))
    return body[1][0][0][1][0]


def get_random_playerkeys():
    body = demodulate(iter(post_to_server("11011000011101000")))
    return (body[1][0][0][1][0], body[1][0][1][0][1][0])


def post_to_server(command):
    req = urllib.request.Request(
        url='https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=' +
        os.environ['ICFPC_API_KEY'],
        data=command.encode('utf-8'),
        method='POST')
    req.add_header('Content-Type', 'text/plain')
    with urllib.request.urlopen(req, timeout=10) as resp:
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
    if first == []:
        first = None
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
