test_hello_world = ¡Hola mundo!
test_hello_user  = ¡Hola, { $userName }!
test_shared_photos =
    { $userName } { $photoCount ->
        [one] ha añadido una nueva foto
       *[other] ha añadido { $photoCount } nuevas fotos
    } de { $userGender ->
        [male] él y su familia
        [female] ella y su familia
       *[other] la familia
    }.
