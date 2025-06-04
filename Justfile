fmt:
	@echo "🧹  Formatting all years…"
	@for dir in */; do \
		if [ -f "$dir/Cargo.toml" ]; then \
			echo "🧹  Formatting $dir"; \
			(cd "$dir" && cargo fmt); \
		fi; \
	done

clippy:
	@echo "📎  Running Clippy on all years…"
	@for dir in */; do \
		if [ -f "$dir/Cargo.toml" ]; then \
			echo "📎  Linting $dir"; \
			(cd "$dir" && cargo clippy --all-targets -- -D warnings); \
		fi; \
	done

check:
	@echo "✅  Running cargo check on all years…"
	@for dir in */; do \
		if [ -f "$dir/Cargo.toml" ]; then \
			echo "✅  Checking $dir"; \
			(cd "$dir" && cargo check); \
		fi; \
	done

test:
	@echo "🧪  Running cargo test on all years…"
	@for dir in */; do \
		if [ -f "$dir/Cargo.toml" ]; then \
			echo "🧪  Testing $dir"; \
			(cd "$dir" && cargo test); \
		fi; \
	done

update:
	@echo "🔄  Updating dependencies for all years…"
	@for dir in */; do \
		if [ -f "$dir/Cargo.toml" ]; then \
			echo "🔄  Updating $dir"; \
			(cd "$dir" && cargo update); \
		fi; \
	done
