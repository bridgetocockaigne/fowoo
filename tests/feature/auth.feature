Feature: Get user info from oauth client

    Scenario Outline: Get user info with code
        Given a code '<code>'
        And a mock client provider
        When I call the provider with the code
        Then I get an OK response

    Examples:
        | code |
        | xxxx |
