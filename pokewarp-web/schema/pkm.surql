---define SCHEMAFULL and PERMISSIONS
DEFINE TABLE pkm SCHEMAFULL
  PERMISSIONS
    FOR select, update WHERE <-boxed.in = [$auth.id];
--- define FIELD's
DEFINE FIELD pkm_id ON TABLE pkm TYPE number;
DEFINE FIELD species ON TABLE pkm TYPE string;
DEFINE FIELD item_id ON TABLE pkm TYPE number;
DEFINE FIELD exp ON TABLE pkm TYPE number;
DEFINE FIELD ability ON TABLE pkm TYPE string;
DEFINE FIELD moves ON TABLE pkm FLEXIBLE TYPE array;
DEFINE FIELD ivs ON TABLE pkm FLEXIBLE TYPE object;
DEFINE FIELD evs ON TABLE pkm FLEXIBLE TYPE object;
DEFINE FIELD nature ON TABLE pkm TYPE string;
DEFINE FIELD nickname ON TABLE pkm TYPE string;
DEFINE FIELD happiness ON TABLE pkm TYPE number;
DEFINE FIELD gender ON TABLE pkm TYPE string;
DEFINE FIELD level ON TABLE pkm TYPE number;
DEFINE FIELD is_egg ON TABLE pkm TYPE bool;
DEFINE FIELD item ON TABLE pkm FLEXIBLE TYPE object;
DEFINE FIELD pokeball ON TABLE pkm FLEXIBLE TYPE object;
DEFINE FIELD ot_name ON TABLE pkm TYPE string;
DEFINE FIELD ot_id ON TABLE pkm TYPE number;
DEFINE FIELD ot_sid ON TABLE pkm TYPE number;