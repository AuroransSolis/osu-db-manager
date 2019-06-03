pub fn print_help_message() {
    println!("\
Usage: osu-db-manager [DATABASE OPTION] [DB FILE PATH] [OPTIONS]\n\
\n\
    Database options:\n    \
        -o  --osu  osu!.db\n    \
        -c  --collection  collection.db\n    \
        -s  --scores  scores.db\n\
    \n\
Input options:\n    \
    -i --interface=[OPTION]  Interface for for merging databases.\n        \
        Options:\n            \
            s/shell  Command interface for merging databases.\n            \
            t/tui  TUI interface for merging databases.\n            \
            none (default)  Merge databases according to other options\n                \
                without user input.\n    \
    -q  --query [QUERY]  Query database with QUERY. Each query term\n        \
        consists of a field name followed by an equals sign and a complete\n        \
        string to search for. Query terms should be separated by a comma\n        \
        without a space. Numbers can be specified as integers to search\n        \
        for decimals with the same whole number value, and decimals will\n        \
        search for an exact value. For instance, to search osu!.db for all\n        \
        beatmaps created by Sotarks with an approach rate of 9.X, the\n        \
        command would be\n            \
            osu-db-manager -o PATH_TO_OSUDB -q CREATOR_NAME=Sotarks,AR=9\n        \
        For information on what you can query for in each database, do\n            \
            osu-db-manager [DATABASE OPTION] [-q/--query] [-h/--help]\n    \
    -S  --show [SHOW]  Show fields from database contents using options\n        \
        in SHOW. The format of SHOW should be the same as a QUERY. For\n        \
        information on what you can show from each database, do\n            \
            osu-db-manager [DATABASE OPTION] [-s/--show] [-h/--help]\n    \
    -m  --merge [OPTION] [PATH TO DB OF SAME TYPE]  Merge two databases\n        \
        of the same type. Only scores.db and collection.db instances can\n        \
        be merged, since osu! is quite good at generating osu!.db. If you\n        \
        need to merge two osu!.db instances, just copy the contents of the\n        \
        song folder from one osu! install to another, open the game, go to\n        \
        singleplayer and press F5. osu! will handle adding the new beatmaps\n        \
        to osu!.db for you.\n        \
        Options:\n           \
            --conflict-resolution=[OPTION]  How to merge duplicate entries.\n           \
            Options:\n               \
                ignore  Ignore duplicates in the second database.\n               \
                replace  Replace duplicates in the first database with entries\
                \n                    \
                    from the second.\n               \
                second-prefix=PREFIX  Use PREFIX as a prefix to duplicates in\n                    \
                    the second database.\n               \
                second-suffix=SUFFIX  Use SUFFIX as a suffix to duplicates in\n                    \
                    the second database.\n               \
                first-prefix=PREFIX  Use PREFIX as a prefix to duplicates in\n                    \
                    the first database.\n               \
                first-suffix=SUFFIX  Use SUFFIX as a suffix to duplicates in\n                    \
                    the first database.\n               \
                merge-subentries(default)  Copy subentries from the second\n                    \
                    database into entries from the second. For instance, with\n                    \
                    scores.db, the score subentries for duplicate beatmaps in\n                    \
                    the second database will be appended to the list of scores\
                    \n                    \
                    in the first database's entry for that beatmap.\n            \
            --checked=PATH-TO-OSUDB  Checks to ensure that beatmaps\n                \
                specified in the second database's entries exist in the\n                \
                first's osu!.db. RECOMMENDED");
}