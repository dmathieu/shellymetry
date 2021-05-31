TOP_LEVEL = $(shell git rev-parse --show-toplevel)
CIRCLECI_DIR = $(TOP_LEVEL)/.circleci

CIRCLECI_CONFIG := $(CIRCLECI_DIR)/config.yml
PROCESSED_CIRCLECI_CONFIG := $(CIRCLECI_DIR)/.processed.yml

.PHONY: help
help: # Prints out help
	@IFS=$$'\n' ; \
	help_lines=(`fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##/:/'`); \
	printf "%-30s %s\n" "target" "help" ; \
	printf "%-30s %s\n" "------" "----" ; \
	for help_line in $${help_lines[@]}; do \
			IFS=$$':' ; \
			help_split=($$help_line) ; \
			help_command=`echo $${help_split[0]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
			help_info=`echo $${help_split[2]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
			printf '\033[36m'; \
			printf "%-30s %s" $$help_command ; \
			printf '\033[0m'; \
			printf "%s\n" $$help_info; \
	done

# Processes the circle ci config locally
$(CIRCLECI_CONFIG):
$(PROCESSED_CIRCLECI_CONFIG): $(CIRCLECI_CONFIG)
	circleci config process $(CIRCLECI_CONFIG) > $(PROCESSED_CIRCLECI_CONFIG)

.PHONY: ci-test
ci-test: ## Runs the ci based test job locally
ci-test: $(PROCESSED_CIRCLECI_CONFIG)
	circleci local execute --job test -c $(PROCESSED_CIRCLECI_CONFIG)
