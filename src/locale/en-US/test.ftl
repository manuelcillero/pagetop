test_hello_world = Hello world!
test_hello_user  = Hello, { $userName }!
test_shared_photos =
    { $userName } { $photoCount ->
        [one] added a new photo
       *[other] added { $photoCount } new photos
    } of { $userGender ->
        [male] him and his family
        [female] her and her family
       *[other] their family
    }.
