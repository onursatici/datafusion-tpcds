DSGEN_PATH := vendor/DSGen-software-code-3.2.0rc1

.PHONY: tpc_ds_image
tpc_ds_image:
	docker build --build-arg TPC_DS_PATH="$(DSGEN_PATH)" -f docker/Dockerfile -t "tpc_ds:$$(git describe --tags)" .

	

