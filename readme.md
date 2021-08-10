psql -d pixelguesser -a -f ./create.sql

- Create
    - exit + leave: quiz will be saved as draft
    - faulty form: give error on fields
    - continue: commit newQuiz
- Rounds
    - _: commit every few seconds with visual feedback
    