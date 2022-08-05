
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
  - ~~Make quizzes searchable by name and description (low)~~
  - ~~Editable icon only shows if user has privileges (high)~~
  - Favourite quizzes and sharing quizzes (low)
  - Hide incomplete quizzes (low)
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
  - ~~Test properly (high)~~
  - ~~Queries should be deterministic aka order by (medium)~~
  - Proper forms and public checkbox (high)
  - More options than pixelate (medium)
  - Leave warning unsaved work (low)
  - ~~Atomic writes upon submit or continuous saves? - if not atomic, reference to the new page of the quiz (low)~~
  - Portrait image rotated (this is due to the EXIF extension being ignored) (high)
  - ~~Cropper has delay before showing image (high)~~
  - ~~Side images are wrong when very-wide (medium)~~
  - ~~Extra information on what's missing on overview click - flash danger red on missing (low)~~
  - Remove images form server if dangling reference (medium)
  - ~~Image similarity system to avoid duplicates on the server (low)~~
  - Handling of no user set in context (see quiz_page return html) (low)
  - ~~Generalised state diffing manager (medium)~~ (indirectly fixed with round/quiz merge)
  - Search web for images (google/unsplash/pixabay) (medium)
  - Drag and drop image, copy paste url, ... (medium)
  - Attribute and download (Photo by <a href="https://unsplash.com/@anniespratt?utm_source=your_app_name&utm_medium=referral">Annie Spratt</a> on <a href="https://unsplash.com/?utm_source=your_app_name&utm_medium=referral">Unsplash</a>)
- Auth
  - ~~Keep logged in (medium)~~
  - Rusty API calls instead of JS SDK (low)
  - Warnings with cookies about same site (low)
  - Style the login interface (medium)
  - Interface to change nickname and other profile stuff (high)
- Backend:
  - ~~Make image size dynamic based on needs (high)~~
  - Security overview (medium)
  - ~~Rewritten image solution with proper scaling (high)~~
  - ~~GraphQL answer handling built-in to api (high)~~
- General
  - Consistent styling (medium)
  - Help/Explain pages (low)
  - Logo (high)

# Known issues
When deleting a quiz, it redirects to the overview. 
Sometimes the quiz is not yet deleted before the page 
is fetched resulting in wrong info shown.


  

