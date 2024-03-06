%.build: src/bin/%.rs
	@echo "Building $@"
	@yarn generate --name $*

week3: mutable_and_shadowing.build data_types.build data_types_2.build functions.build control_flow_3.build

