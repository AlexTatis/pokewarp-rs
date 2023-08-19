DEFINE TABLE boxed SCHEMAFULL  
PERMISSIONS
    FOR select, update WHERE in.id = $auth.id;
DEFINE FIELD in ON TABLE boxed TYPE record(user);
DEFINE FIELD out ON TABLE boxed TYPE record(pkm);
DEFINE FIELD box ON TABLE boxed TYPE number;
DEFINE FIELD slot ON TABLE boxed TYPE number;