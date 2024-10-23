DSGEN_PATH := vendor/DSGen-software-code-3.2.0rc1
SCALE_FACTOR := 1

.PHONY: tpc_ds_image
tpc_ds_image:
	docker build --build-arg TPC_DS_PATH="$(DSGEN_PATH)" -f docker/Dockerfile -t "tpc_ds:$$(git describe --tags)" .

.PHONY: generate_queries
generate_queries: tpc_ds_image
	mkdir -p queries
	docker run --volume ./queries:/tmp "tpc_ds:$$(git describe --tags)" ./dsqgen\
	  -scale "$(SCALE_FACTOR)"\
	  -directory ../query_templates\
	  -input ../query_templates/templates.lst\
	  -dialect postgres\
	  -output_dir /tmp

.PHONY: generate_data
generate_data: tpc_ds_image
	mkdir -p data
	docker run --volume ./data:/tmp "tpc_ds:$$(git describe --tags)"\
	  ./dsdgen -scale 1 -verbose y -f -dir /tmp
