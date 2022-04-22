
# PixelGuesser
PixelGuesser is a party game where players try to guess the contents of an
image as quickly as possible. The image starts out heavily pixelated with 
and slowly grows more detailed over time. The game is meant to played in 
real-life, but some support for on-line play could be added later. The 
intent is that the website is opened on some large screen that can be seen 
by everyone, there we can choose a quiz that we want to play. Someone can 
then become the 'quiz master' by scanning the QR-code on the screen. That 
person will then see the right answers and can select the player that 
guessed first.

# Status
PixelGuesser is a hobby project that is currently in heavy development.
The project is meant as a learning experience to familiarize myself with larger projects.
The basic game is working, but I want some more features and better ease 
of use before I release it. Once I am happy with the state of the project,
it will be released on the website www.pixelguesser.com.

# TODO
- ~~Use Code struct in sessions for type security~~
- Overview 
  - ~~Make quizzes searchable by name and description. (low)~~
  - ~~Editable icon only shows if user has privileges. (high)~~
  - Favourite quizzes and sharing quizzes. (low)
- Host
  - ~~Show the explanation (high)~~
  - ~~Show the points/guesses before every round (high)~~
  - ~~Show score by default after every round (high)~~
  - Show error with back arrow on connection failure (medium)
  - ~~Show warning when trying to leave the page (high)~~
  - Show warning also when back is pressed (medium)
  - Streak bonus (medium)
- Manage
  - Edit points interface, in case of erroneous input (medium)
  - Also need to access the quiz when not owner and not public (high)
- Create
  - Test properly (high)
  - Queries should be deterministic aka order by (medium)
  - Proper forms and public checkbox (high)
  - More options than pixelate (medium)
- Auth
  - ~~Keep logged in (medium)~~
  - Rusty API calls instead of JS SDK (low)
  - Style the login interface (medium)
  - Interface to change nickname and other profile stuff (high)
- Backend:
  - ~~Make image size dynamic based on needs (high)~~
  - Security overview (medium)
- General
  - Consistent styling (medium)
  - Help/Explain pages (low)
  - Logo (high)


  

