test-hello-world = ¡Hola mundo!
test-hello-user  = ¡Hola, { $userName }!
test-shared-photos =
    { $userName } { $photoCount ->
        [one] ha añadido una nueva foto
       *[other] ha añadido { $photoCount } nuevas fotos
    } de { $userGender ->
        [male] él y su familia
        [female] ella y su familia
       *[other] la familia
    }.
