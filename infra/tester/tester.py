import os
import subprocess
import sys
import urllib.request
import logging

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
    if len(sys.argv) == 4:
        if sys.argv[1] == 'tutorial':
            pk = get_tutorial_playerkey(int(sys.argv[2]))
            print("tester: PlayerKey %d" % (pk, ))
            run_bot(sys.argv[3], pk, True)
            return
        elif sys.argv[1] == 'battle':
            pks = get_random_playerkeys()
            print("tester: PlayerKey %d %d" % pks)
            # NOTE: Untested
            t = threading.Thread(target=run_bot,
                                 args=(sys.argv[2], pks[0], False))
            t.start()
            run_bot(sys.argv[3], pks[1], False)
            t.join()
            return

    print("tester.py tutorial NUM BINARY_PATH")
    print("tester.py battle BINARY_PATH BINARY_PATH")
    sys.exit(1)


def run_bot(main_program, player_key, is_tutorial):
    modified_env = os.environ.copy()
    if is_tutorial:
        modified_env["TUTORIAL_MODE"] = "1"
    p = subprocess.Popen([main_program, str(player_key)],
                         bufsize=0,
                         stdin=subprocess.PIPE,
                         stdout=subprocess.PIPE,
                         env=modified_env,
                         close_fds=True,
                         encoding='utf-8')
    (child_stdin, child_stdout) = (p.stdin, p.stdout)
    while True:
        line = child_stdout.readline().strip()
        if line == '':
            return
        logging.info('tester: send: %s', line)
        body = post_to_server(line)
        logging.info('tester: recv: %s', body)
        child_stdin.write(body + '\n')


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
