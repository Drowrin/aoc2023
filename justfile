set export := true

GREEN := '\033[0;032m'
PURPLE := '\033[0;035m'
NO_COLOR := '\033[0m'

[private]
@default:
    just --list

[no-exit-message]
[private]
validate day:
    #!python
    import sys
    import re
    if re.fullmatch(r'(0[1-9]|1\d|2[0-5])[a-z]', "{{ day }}") is None:
        sys.exit("'day' must be a two digit number between 01 and 25 followed by a lowercase letter")

@test day: (validate day)
    echo -e "${GREEN}Testing solution on example input${NO_COLOR}"
    cargo test -q -p day{{ day }}

@run day flags="": (validate day) (test day)
    echo -e "${GREEN}Running solution on puzzle input${NO_COLOR}"
    cargo run -q -p 'day{{ day }}' {{ flags }}

@new day from="": (validate day)
    echo -e "${GREEN}Generating directory ./day{{ day }}${NO_COLOR}"
    cargo init -q 'day{{ day }}'
    cp -r template/* 'day{{ day }}/'
    if [ "{{ from }}" != "" ]; then just transfer {{ from }} {{ day }}; fi

@transfer day to: (validate day) (validate to)
    echo -e "${GREEN}Transferring base from day{{ day }} to day{{ to }}${NO_COLOR}"
    cp "day{{ day }}/src/solution.rs" "day{{ to }}/src/solution.rs"
    cp "day{{ day }}/data/example.txt" "day{{ to }}/data/example.txt"
    cp "day{{ day }}/data/input.txt" "day{{ to }}/data/input.txt"
    echo -e "${PURPLE}Don't forget to populate ./day{{ to }}/data/answer.txt${NO_COLOR}"
    