# shorten_numbers
A small cli tool to shorten numbers

## Usage example

   ```shell
   $ echo foo 987654321 1234567890 1236 456 bar | sn
   foo 987_654_321 1_234_567_890 1_236 456 bar
   $ echo foo 987654321 1234567890 1236 456 bar | sn -s
   foo 988M 1G 1k 456 bar
   ```
