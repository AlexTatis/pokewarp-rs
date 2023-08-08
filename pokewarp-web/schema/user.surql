---define SCHEMAFULL and PERMISSIONS
DEFINE TABLE user SCHEMAFULL
  PERMISSIONS
    FOR select, update WHERE id = $auth.id, 
    FOR create, delete NONE;
--- define FIELD's
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD pass ON user TYPE string;
--- define INDEX's
DEFINE INDEX idx_user ON user COLUMNS name UNIQUE;

-- define SCOPE
DEFINE SCOPE allusers
  -- the JWT session will be valid for 14 days
  SESSION 14d
  -- The optional SIGNUP clause will be run when calling the signup method for this scope
  -- It is designed to create or add a new record to the database.
  -- If set, it needs to return a record or a record id
  -- The variables can be passed in to the signin method
  SIGNUP ( CREATE user SET name = $name, pass = crypto::argon2::generate($pass) )
  -- The optional SIGNIN clause will be run when calling the signin method for this scope
  -- It is designed to check if a record exists in the database.
  -- If set, it needs to return a record or a record id
  -- The variables can be passed in to the signin method
  SIGNIN ( SELECT * FROM user WHERE name = $name AND crypto::argon2::compare(pass, $pass) )
  -- this optional clause will be run when calling the signup method for this scope