test-hello-world = Hello world!
test-hello-user  = Hello, { $userName }!
test-shared-photos =
    { $userName } { $photoCount ->
        [one] added a new photo
       *[other] added { $photoCount } new photos
    } of { $userGender ->
        [male] him and his family
        [female] her and her family
       *[other] their family
    }.
