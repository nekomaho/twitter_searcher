.PHONY: docker/build_binary
docker/build_binary:
	docker build -t twitter_searcher_for_build .
	docker run --name twitter_searcher_for_build -it twitter_searcher_for_build sleep 0
	docker cp twitter_searcher_for_build:/home/build/target/release/twitter_searcher .
	docker stop twitter_searcher_for_build
	docker rm twitter_searcher_for_build
