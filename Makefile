%.build: src/bin/%.rs
	@echo "Building $@"
	@yarn generate --name $*

%.deploy: %.build
	@echo "Deploying $@"
	@yarn deploy --name $*

week3=mutable_and_shadowing data_types data_types_2 functions control_flow_3

week3.build: $(addsuffix .build, $(week3))
	@echo "Building week3"

week3.deploy: $(addsuffix .deploy, $(week3))
	@echo "Deploying week3"

week4=ownership reference

week4.build: $(addsuffix .build, $(week4))
	@echo "Building week4"

week4.deploy: $(addsuffix .deploy, $(week4))
	@echo "Deploying week4"
