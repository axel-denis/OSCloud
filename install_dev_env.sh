echo "Would you like to install all (y) or just sample data (n) ?"

read yesorno

if [ "$yesorno" = y ]; then
  rustup update
  rustup component add clippy
  rustup component add rustfmt
  cargo install --force cargo-make
  cargo install --force cargo-watch
  # sudo apt-get install libpq-dev
fi

mkdir -p database
printf "[
  {
    \"id\": 0,
    \"name\": \"Axel\",
    \"type\": \"admin\",
    \"password\": \"password\"
  },
  {
    \"id\": 1,
    \"name\": \"Arthur\",
    \"type\": \"user\",
    \"password\": \"mdp\"
  }
]" > database/users.json

printf "ACCESS_TOKEN_SECRET=$(uuidgen)" > ".env"
echo "installed. Your actual access token secret is an uuid. \
You can change it to your own if needed"
