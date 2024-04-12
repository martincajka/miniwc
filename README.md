App

CLI args:
coount bytes: -c
count words: -w
count lines: -l
count characters: -m

If no arguments are passed, the program should count bytes, words, lines input.

Input:

if specified in command as path to file as e.g.: test.txt
if not specified, read from stdin

-c, -l, -w, -m + <file_path>:
example:miniwc -c test.txt
output: 100 test.txt

only <file_path>:
example:miniwc test.txt
output: 100 25 5 test.txt -> (bytes, words, lines)

stdin:
example: cat test.txt | miniwc -c
output: 100 test.txt

FLOW:
Handle unknown arguments???
  print error message


Correct input can look like:
miniwc -c test.txt
miniwc -cw test.txt
miniwc test.txt
cat test.txt | miniwc -c


check if first argument starts with '-'
  YES: check if it contains only a valid arguments (in c, w, l, m) - make them as input for counting function e.g.: count_result(args: String, input: String) -> String

    YES: check if second argument is file path
      YES: read file and count
      NO: read from stdin and count

  NO: read as file path and count
   - throw error if file does not exist


is_query_valid(query: String) -> Boolean
is_file_path_valid(file_path: String) -> Boolean
read_file(file_path: String) -> String
count_result(args: String, input: String) -> String
count_bytes(input: String) -> String
count_words(input: String) -> String
count_lines(input: String) -> String
count_characters(input: String) -> String
print_result(result: String) -> Void

If 1. does not start with '-' -> config = cwl AND file_path = 1. arg -> read file and count
If 1. starts with '-' -> config = 1. arg AND 2. arg != empty -> 2. arg = file_path -> read file and count
If 1. starts with '-' -> config = 1. arg AND 2. arg == empty -> read from stdin and count
