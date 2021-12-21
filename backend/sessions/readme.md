# Sessions

This is an independent crate made to manage sessions for the pixelguesser game.
A session contains information on the state of the game as well as the **host** for the game 
(e.g. a smart tv) and the **manager** (e.g. a mobile device of the quiz master).

The requirements of this system are as follows:
1) The **host** creates a session for a certain quiz.
   1) Every session is inherently linked to a **host**, if the host disconnects, the session will be over.
2) The server responds with a secret id for the session which the host should display on the screen.
   1) The code should be secure and easily entered into any device.
3) The **manager** can join by entering the secret code.
   1) There can only be a single manager.
   2) When a manager disconnects, the host should show the secret code again.

To achieve these requirements we need a way to detect whether both the host or manager are connected.
We will need the following functions:

    Host(quiz_id) -> secret_key
    # 
    Manage(secret_key) -> (session_data, action)
    Leave
    # Leave should work for both host and manage without any extra info.

    Update(secret_key, action) -> (session_data, action)
    # The session should always check if an action is valid and legal

    notifications -> (session_data, action)
    # Notifications can happen at any time and should overwrite the state.



