
# PixelGuesser
PixelGuesser is a real life party gam where players try to guess the contents of an
image as quickly as possible. The image starts out heavily pixelated with and slowly
grows more detailed over time.

# Config
**serve index:** run --manifest-path backend/spar/Cargo.toml -- -p 8900 --folder data/static

**serve images:** run --manifest-path backend/imager/Cargo.toml -- -p 8901 --folder data/images

**start session manager:** run --features "full" --manifest-path backend/sessions/Cargo.toml -- -p 8902

**generate frontend:** build --target web --dev --out-name wasm --out-dir ../data/static

# TODO
- Overview 
  - Make quizzes searchable by name and description. (low)
  - ~~Editable icon only shows if user has privileges. (high)~~
  - Favourite quizzes and quiz sharing quizzes. (low)
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
  - Make image size dynamic based on needs (high)
  - Security overview (medium)
- General
  - Consistent styling (medium)
  - Help/Explain pages (low)
  - Logo (high)
  
# Architecture
This project uses WASM to create a single-page application. This project 
seeks to be as modular as possible. In the backend we have multiple components
for serving the SPA, dynamic imaging, managing sessions, graphql API and an 
auth server. This project uses graphql API we use Hasura and the auth server is 
Auth0.


  

