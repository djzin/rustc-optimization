RUSTFLAGS = -C opt-level=3 -C target-cpu=native -C panic=abort

tests: nullary-tests unary-tests

nullary-tests: $(addprefix out/nullary/, $(addsuffix .s, $(basename $(notdir $(wildcard src/nullary/*.rs)))))
	@echo "using RUSTFLAGS=$(RUSTFLAGS)"
	@failures=0; successes=0; \
	for file in $^; do \
		if [ 2 -ne $$(wc -l <$$file) ]; then \
			echo "[0;31mtest $$file failed[0m - test output:"; \
			cat $$file; \
			failures=$$((failures + 1)); \
		else \
			echo "[0;32mtest $$file passed[0m"; \
			successes=$$((successes + 1)); \
		fi \
	done; \
	echo "nullary tests run - [0;32m$$successes successes[0m, [0;31m$$failures failures[0m";

unary-tests: $(addprefix out/unary/, $(addsuffix .s, $(basename $(notdir $(wildcard src/unary/*.rs)))))
	@echo "using RUSTFLAGS=$(RUSTFLAGS)"
	@failures=0; successes=0; \
	for file in $^; do \
		if [ 3 -ne $$(wc -l <$$file) ]; then \
			echo "[0;31mtest $$file failed[0m - test output:"; \
			cat $$file; \
			failures=$$((failures + 1)); \
		else \
			echo "[0;32mtest $$file passed[0m"; \
			successes=$$((successes + 1)); \
		fi \
	done; \
	echo "unary tests run - [0;32m$$successes successes[0m, [0;31m$$failures failures[0m";

out/nullary/%.s: src/nullary/%.rs
	@mkdir -p out/nullary
	rustc $< --crate-type=lib --emit=asm $(RUSTFLAGS) -o $@
	@sed -n -i -e '/^$(notdir $(basename $@))/,/retq/p' $@

out/unary/%.s: src/unary/%.rs
	@mkdir -p out/unary
	rustc $< --crate-type=lib --emit=asm $(RUSTFLAGS) -o $@
	@sed -n -i -e '/^$(notdir $(basename $@))/,/retq/p' $@

clean:
	rm -r out
