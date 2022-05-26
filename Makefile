src_target = target/wasm32-unknown-unknown/release
root_directory = ./

# liquid_locker_des_wasm = liquid-locker/liquid-locker-tests/wasm
# liquid_helper_des_wasm = liquid-helper/liquid-helper-tests/wasm
minter_des_wasm = minter/minter-tests/wasm
gauge_controller_des_wasm = gauge-controller/gauge-controller-tests/wasm
reward_only_gauge_des_wasm = reward-only-gauge/reward-only-gauge-tests/wasm
vesting_escrow_des_wasm = vesting-escrow/vesting-escrow-tests/wasm
vesting_escrow_factory_des_wasm = vesting-escrow-factory/vesting-escrow-factory-tests/wasm

wasm_src_path = target/wasm32-unknown-unknown/release/
wasm_dest_voting_escrow_path = ./voting-escrow/voting-escrow-tests/wasm/
wasm_dest_fee_distributor_path = ./fee-distributor/fee-distributor-tests/wasm/
wasm_dest_liquidity_gauge_reward_path = ./liquidity-gauge-reward/liquidity-gauge-reward-tests/wasm/
wasm_dest_erc20_path = ./erc20/erc20-tests/wasm/
wasm_dest_liquidity_gauge_reward_wrapper_path = ./liquidity-gauge-reward-wrapper/liquidity-gauge-reward-wrapper-tests/wasm/
wasm_dest_liquidity_gauge_wrapper_path = ./liquidity-gauge-wrapper/liquidity-gauge-wrapper-tests/wasm/


prepare:
	rustup target add wasm32-unknown-unknown

build-session-code:
	cargo build --release -p session-code --target wasm32-unknown-unknown
build-liquidity-gauge-reward-wrapper-session-code:
	cargo build --release -p liquidity-gauge-reward-wrapper-session-code --target wasm32-unknown-unknown
build-liquidity-gauge-wrapper-session-code:
	cargo build --release -p liquidity-gauge-wrapper-session-code --target wasm32-unknown-unknown	
build-contract-erc20:
	cargo build --release -p erc20 -p erc20-proxy --target wasm32-unknown-unknown
build-contract-minter:
	cargo build --release -p minter -p minter-proxy --target wasm32-unknown-unknown
build-contract-gauge-controller:
	cargo build --release -p gauge-controller -p gauge-controller-proxy --target wasm32-unknown-unknown
build-contract-reward-only-gauge:
	cargo build --release -p reward-only-gauge -p reward-only-gauge-proxy --target wasm32-unknown-unknown
build-contract-vesting-escrow:
	cargo build --release -p vesting-escrow -p vesting-escrow-proxy --target wasm32-unknown-unknown
build-contract-vesting-escrow-factory:
	cargo build --release -p vesting-escrow-factory -p vesting-escrow-factory-proxy --target wasm32-unknown-unknown
build-contract-voting-escrow:
	cargo build --release -p voting-escrow -p erc20 --target wasm32-unknown-unknown
build-contract-fee-distributor:
	cargo build --release -p fee-distributor --target wasm32-unknown-unknown
build-contract-liquidity-gauge-reward:
	cargo build --release -p liquidity-gauge-reward --target wasm32-unknown-unknown
build-contract-erc20-crv:
	cargo build --release -p erc20_crv --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/erc20_crv.wasm 2>/dev/null | true
build-erc20-crv-session-code:
	cargo build --release -p erc20-crv-session-code --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/erc20-crv-session-code.wasm 2>/dev/null | true
build-contract-vesting-escrow-simple:
	cargo build --release -p vesting_escrow_simple --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/vesting_escrow_simple.wasm 2>/dev/null | true
build-contract-liquidity-gauge-reward-wrapper:
	cargo build --release -p liquidity-gauge-reward-wrapper --target wasm32-unknown-unknown
build-contract-liquidity-gauge-wrapper:
	cargo build --release -p liquidity-gauge-wrapper --target wasm32-unknown-unknown
build-contract-curve-token-v1:
	cargo build --release -p curve_token_v1 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/curve_token_v1.wasm 2>/dev/null | true
build-contract-curve-token-v2:
	cargo build --release -p curve_token_v2 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/curve_token_v2.wasm 2>/dev/null | true
build-contract-curve-token-v3:
	cargo build --release -p curve_token_v3 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/curve_token_v3.wasm 2>/dev/null | true





test-only-minter:
	cargo test -p minter-tests
test-only-gauge-controller:
	cargo test -p gauge-controller-tests
test-only-reward-only-gauge:
	cargo test -p reward-only-gauge-tests
test-only-vesting-escrow:
	cargo test -p vesting-escrow-tests
test-only-vesting-escrow-factory:
	cargo test -p vesting-escrow-factory-tests
test-only-voting-escrow:
	cargo test -p voting-escrow-tests
test-only-fee-distributor:
	cargo test -p fee-distributor-tests
test-only-liquidity-gauge-reward:
	cargo test -p liquidity-gauge-reward-tests
test-only-erc20-crv:
	cargo test -p erc20_crv_tests -- --nocapture
test-only-vesting-escrow-simple:
	cargo test -p vesting_escrow_simple_tests -- --nocapture
test-only-erc20:
	cargo test -p erc20-tests	
test-only-liquidity-gauge-reward-wrapper:
	cargo test -p liquidity-gauge-reward-wrapper-tests
test-only-liquidity-gauge-wrapper:
	cargo test -p liquidity-gauge-wrapper-tests
test-only-curve-token-v1:
	cargo test -p curve_token_v1_tests -- --nocapture
test-only-curve-token-v2:
	cargo test -p curve_token_v2_tests -- --nocapture
test-only-curve-token-v3:
	cargo test -p curve_token_v3_tests -- --nocapture
	



copy-wasm-file-minter:
	cp ${src_target}/minter-token.wasm ${minter_des_wasm}
	cp ${src_target}/minter-proxy-token.wasm ${minter_des_wasm}
copy-wasm-file-gauge-controller:
	cp ${src_target}/gauge-controller-token.wasm ${gauge_controller_des_wasm}
	cp ${src_target}/gauge-controller-proxy-token.wasm ${gauge_controller_des_wasm}
copy-wasm-file-reward-only-gauge:
	cp ${src_target}/reward-only-gauge-token.wasm ${reward_only_gauge_des_wasm}
	cp ${src_target}/reward-only-gauge-proxy-token.wasm ${reward_only_gauge_des_wasm}
copy-wasm-file-vesting-escrow:
	cp ${src_target}/vesting-escrow-token.wasm ${vesting_escrow_des_wasm}
	cp ${src_target}/vesting-escrow-proxy-token.wasm ${vesting_escrow_des_wasm}
copy-wasm-file-vesting-escrow-factory:
	cp ${src_target}/vesting-escrow-factory-token.wasm ${vesting_escrow_factory_des_wasm}
	cp ${src_target}/vesting-escrow-factory-proxy-token.wasm ${vesting_escrow_factory_des_wasm}
copy-wasm-file-voting-escrow:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_voting_escrow_path}
copy-wasm-file-fee-distributor:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_fee_distributor_path}
copy-wasm-file-liquidity-gauge-reward:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_liquidity_gauge_reward_path}
copy-wasm-file-erc20:
	cp ${root_directory}${wasm_src_path}erc20-token.wasm ${wasm_dest_erc20_path}
	cp ${root_directory}${wasm_src_path}erc20-proxy-token.wasm ${wasm_dest_erc20_path}
copy-wasm-file-liquidity-gauge-reward-wrapper:
	cp ${root_directory}${wasm_src_path}erc20-token.wasm ${wasm_dest_liquidity_gauge_reward_wrapper_path}
	cp ${root_directory}${wasm_src_path}liquidity-gauge-reward-wrapper.wasm ${wasm_dest_liquidity_gauge_reward_wrapper_path}
	cp ${root_directory}${wasm_src_path}liquidity-gauge-reward-wrapper-session-code.wasm ${wasm_dest_liquidity_gauge_reward_wrapper_path}
	cp ${root_directory}${wasm_src_path}minter-token.wasm ${wasm_dest_liquidity_gauge_reward_wrapper_path}
	cp ${root_directory}${wasm_src_path}liquidity-gauge-reward.wasm ${wasm_dest_liquidity_gauge_reward_wrapper_path}

copy-wasm-file-liquidity-gauge-wrapper:
	cp ${root_directory}${wasm_src_path}erc20-token.wasm ${wasm_dest_liquidity_gauge_wrapper_path}
	cp ${root_directory}${wasm_src_path}liquidity-gauge-wrapper.wasm ${wasm_dest_liquidity_gauge_wrapper_path}
	cp ${root_directory}${wasm_src_path}liquidity-gauge-wrapper-session-code.wasm ${wasm_dest_liquidity_gauge_wrapper_path}
	cp ${root_directory}${wasm_src_path}minter-token.wasm ${wasm_dest_liquidity_gauge_wrapper_path}
	cp ${root_directory}${wasm_src_path}liquidity-gauge-reward.wasm ${wasm_dest_liquidity_gauge_wrapper_path}

copy-wasm-file-erc20-crv:
	cp target/wasm32-unknown-unknown/release/*.wasm erc20-crv/erc20_crv_tests/wasm
copy-wasm-file-vesting-escrow-simple:
	cp target/wasm32-unknown-unknown/release/*.wasm vesting-escrow-simple/vesting-escrow-simple-tests/wasm
copy-wasm-file-curve-token-v1:
	cp target/wasm32-unknown-unknown/release/*.wasm curve-token-v1/curve-token-v1-tests/wasm
copy-wasm-file-curve-token-v2:
	cp target/wasm32-unknown-unknown/release/*.wasm curve-token-v2/curve-token-v2-tests/wasm
copy-wasm-file-curve-token-v3:
	cp target/wasm32-unknown-unknown/release/*.wasm curve-token-v3/curve-token-v3-tests/wasm

test-minter:
	make build-contract-minter && make copy-wasm-file-minter
test-gauge-controller:
	make build-contract-gauge-controller && make copy-wasm-file-gauge-controller
test-reward-only-gauge:
	make build-contract-reward-only-gauge && make copy-wasm-file-reward-only-gauge
test-vesting-escrow:
	make build-contract-vesting-escrow && make copy-wasm-file-vesting-escrow
test-vesting-escrow-factory:
	make build-contract-vesting-escrow-factory && make copy-wasm-file-vesting-escrow-factory
test-voting-escrow:
	make build-session-code && make build-contract-voting-escrow && make copy-wasm-file-voting-escrow
test-fee-distributor:
	make build-session-code && make build-contract-fee-distributor && make copy-wasm-file-fee-distributor 
test-liquidity-gauge-reward:
	make build-session-code && make build-contract-liquidity-gauge-reward && make copy-wasm-file-liquidity-gauge-reward 
test-erc20-crv: 
	make build-contract-erc20-crv && make build-erc20-crv-session-code && make copy-wasm-file-erc20-crv
test-vesting-escrow-simple: 
	make build-contract-vesting-escrow-simple && make copy-wasm-file-vesting-escrow-simple 
test-liquidity-gauge-reward-wrapper:
	make build-contract-erc20 && make build-contract-minter && make build-contract-liquidity-gauge-reward && make build-liquidity-gauge-reward-wrapper-session-code && make build-contract-liquidity-gauge-reward-wrapper && make copy-wasm-file-liquidity-gauge-reward-wrapper
test-liquidity-gauge-wrapper:
	make build-contract-erc20 && make build-contract-minter && make build-contract-liquidity-gauge-reward && make build-liquidity-gauge-wrapper-session-code && make build-contract-liquidity-gauge-wrapper && make copy-wasm-file-liquidity-gauge-wrapper
test-erc20:
	make build-contract-erc20 && make copy-wasm-file-erc20
test-curve-token-v1: 
	make build-contract-curve-token-v1 && make copy-wasm-file-curve-token-v1
test-curve-token-v2: 
	make build-contract-curve-token-v2 && make copy-wasm-file-curve-token-v2
test-curve-token-v3: 
	make build-contract-curve-token-v3 && make copy-wasm-file-curve-token-v3

all:
	make test-erc20 && make test-only-erc20
	make test-erc20-crv && make test-only-erc20-crv
	make test-minter && make test-only-minter
	make test-gauge-controller && make test-only-gauge-controller
	make test-reward-only-gauge && make test-only-reward-only-gauge
	make test-vesting-escrow && make test-only-vesting-escrow
	make test-vesting-escrow-factory && make test-only-vesting-escrow-factory
	make test-voting-escrow && make test-only-voting-escrow
	make test-fee-distributor && make test-only-fee-distributor
	make test-liquidity-gauge-reward && make test-only-liquidity-gauge-reward
	make test-vesting-escrow-simple && make test-only-vesting-escrow-simple
	make test-liquidity-gauge-reward-wrapper && make test-only-liquidity-gauge-reward-wrapper
	make test-curve-token-v1 && make test-only-curve-token-v1
	make test-curve-token-v2 && make test-only-curve-token-v2
	make test-curve-token-v3 && make test-only-curve-token-v3



clean:
	cargo clean
	rm -rf minter/minter-tests/wasm/*.wasm
	rm -rf gauge-controller/gauge-controller-tests/wasm/*.wasm
	rm -rf reward-only-gauge/reward-only-gauge-tests/wasm/*.wasm
	rm -rf vesting-escrow/vesting-escrow-tests/wasm/*.wasm
	rm -rf vesting-escrow-factory/vesting-escrow-factory-tests/wasm/*.wasm
	rm -rf vesting_escrow_simple_tests/wasm/*.wasm
	rm -rf ${wasm_dest_fee_distributor_path}*.wasm
	rm -rf ${wasm_dest_voting_escrow_path}*.wasm
	rm -rf ${wasm_dest_liquidity_gauge_reward_path}*.wasm
	rm -rf curve_token_v1_tests/wasm/*.wasm
	rm -rf curve_token_v2_tests/wasm/*.wasm
	rm -rf curve_token_v3_tests/wasm/*.wasm

	rm -rf erc20_crv_tests/wasm/*.wasm
	rm -rf Cargo.lock

lint: clippy
	cargo fmt --all
check-lint: clippy
	cargo fmt --all -- --check
clippy:
	cargo clippy --all-targets --all -- -D warnings


