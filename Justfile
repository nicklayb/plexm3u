version := `grep -m1 '^version\s*=' Cargo.toml | cut -d '"' -f2`

user := 'nboisvert'
image_name := 'plexm3u'

build:
  cargo build

run +ARGS:
  cargo run -- {{ARGS}}

build_container:
  nix build .#container
  docker load < ./result

tag_container:
  docker tag {{image_name}}:latest {{user}}/{{image_name}}:{{version}}
  docker tag {{image_name}}:latest {{user}}/{{image_name}}:latest

push_container:
  docker push {{user}}/{{image_name}}:{{version}}
  docker push {{user}}/{{image_name}}:latest

release_docker: build_container tag_container push_container

