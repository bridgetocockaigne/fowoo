Feature: A user is able to login

    Scenario: A user vists the homepage
        When the user visits the homepage
        Then the return status is 200

    Scenario: A user vists the login page
        When the user vists the login page
        Then the return status is 200

    Scenario: A user selects the oauth provider
        When the user visits the oauth provider page
        Then the return status is 307
