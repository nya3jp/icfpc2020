import subprocess
import sys
import urllib.request
import logging

logging.basicConfig(level=logging.INFO)


def main():
    server_url = sys.argv[1]
    player_key = sys.argv[2]
    main_program = sys.argv[3]
    logging.info('ServerUrl: %s; PlayerKey: %s; MainProgram: %s', server_url,
                 player_key, main_program)

    p = subprocess.Popen([main_program, player_key],
                         bufsize=0,
                         stdin=subprocess.PIPE,
                         stdout=subprocess.PIPE,
                         close_fds=True,
                         encoding='utf-8')
    (child_stdin, child_stdout) = (p.stdin, p.stdout)
    while True:
        line = child_stdout.readline().strip()
        if line == '':
            return
        logging.info('interact: sending %s', line)
        req = urllib.request.Request(url=server_url + '/aliens/send',
                                     data=line.encode('utf-8'),
                                     method='POST',
                                     timeout=10)
        req.add_header('Content-Type', 'text/plain')
        with urllib.request.urlopen(req) as resp:
            if resp.status != 200:
                logging.error('interact: non-200 response %s', resp)
                sys.exit(1)
            body = resp.read().decode('utf-8').strip()
            logging.info('interact: received %s', body)
            child_stdin.write(body + '\n')


if __name__ == '__main__':
    main()
