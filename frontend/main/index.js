let auth0 = null;

window.init_auth = async (domain, client_id) => {
    const args = {
        "domain": domain,
        "client_id": client_id,
        "audience": "https://hasura.io/learn",
        "cacheLocation": 'localstorage',
    }
    console.log("init auth");

    auth0 = await createAuth0Client(args);

    const query = window.location.search;
    if (query.includes("code=") && query.includes("state=")) {
        await auth0.handleRedirectCallback();
    }

    if (await auth0.isAuthenticated()) {
        let user = await auth0.getUser();
        user["token"] = "Bearer " + await auth0.getTokenSilently();
        return user;
    }
}

window.redirect_to_signup = async () => {
    await auth0.loginWithRedirect({
        redirect_uri: window.location.origin,
        screen_hint: "signup"
    });
}

window.redirect_to_login = async () => {
    await auth0.loginWithRedirect({
        redirect_uri: window.location.origin,
    });
}

window.logout = () => {
    auth0.logout({
        returnTo: window.location.origin
    });
}