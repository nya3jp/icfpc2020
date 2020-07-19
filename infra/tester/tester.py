import argparse
import logging
import os
import subprocess
import sys
import threading
import urllib.request

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
                        help='log file for the first bot')
    parser.add_argument('--logfile2', help='log file for the second bot')
    parser.add_argument('--logprefix',
                        '--logprefix1',
                        help='log prefix for the first bot')
    parser.add_argument('--logprefix2', help='log prefix for the second bot')

    args = parser.parse_args()

    if args.subcommand == 'tutorial':
        outfile = sys.stderr
        if args.logfile:
            outfile = open(args.logfile, mode='a')
            print("============", file=outfile)
        outprefix = None
        if args.logprefix:
            outprefix = args.logprefix

        pk = get_tutorial_playerkey(int(args.arg1))
        print("tester: PlayerKey %d" % (pk, ))
        run_bot(args.arg2, pk, True, outprefix, outfile)
        return
    elif args.subcommand == 'battle':
        outfile1 = sys.stderr
        if args.logfile1:
            outfile1 = open(args.logfile1, mode='a')
            print("============", file=outfile1)
        outprefix1 = 'bot1'
        if args.logprefix1:
            outprefix1 = args.logprefix2
        outfile2 = sys.stderr
        if args.logfile2:
            outfile2 = open(args.logfile2, mode='a')
            print("============", file=outfile2)
        outprefix2 = 'bot2'
        if args.logprefix2:
            outprefix2 = args.logprefix2

        pks = get_random_playerkeys()
        print("tester: PlayerKey %d %d" % pks)
        # NOTE: Untested
        t = threading.Thread(target=run_bot,
                             args=(args.arg1, pks[0], False, outprefix1,
                                   outfile1))
        t.start()
        run_bot(args.arg2, pks[1], False, outprfix2, outfile2)
        t.join()
        return

    print("tester.py tutorial NUM BINARY_PATH")
    print("tester.py battle BINARY_PATH BINARY_PATH")
    sys.exit(1)


def wrap_err(err_pipe, prefix, outfile):
    for line in err_pipe:
        line = line.rstrip()
        if prefix:
            line = "[%s] %s" % (prefix, line)
        print(line, file=outfile)


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
    with urllib.request.urlopen(req) as resp:
        if resp.status != 200:
            logging.error('tester: non-200 response %s', resp)
            sys.exit(1)
        return resp.read().decode('utf-8').strip()


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
