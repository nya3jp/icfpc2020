# Copyright 2020 Google LLC
# Copyright 2020 Team Spacecat
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

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
                                     method='POST')
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
