rec_assets = $(wildcard rec.*.tape) $(wildcard **/rec.*.tape)
screenshot_assets = $(wildcard screenshot.*.tape) $(wildcard **/screenshot.*.tape)
recs = $(rec_assets:rec.%.tape=asset.%.gif)
screenshots = $(screenshot_assets:screenshot.%.tape=asset.%.png)

prefix_file = static.prefix.tape
suffix_file = static.suffix.tape
temp_tape = temp.tape
temp_out  = out.gif

all: $(recs) $(screenshots)

asset.%.gif: rec.%.tape
	@echo "Output $@"             >  ${temp_tape}
	@cat  ${prefix_file}          >> ${temp_tape}
	@cat  $<                      >> ${temp_tape}
	@cat  ${suffix_file}          >> ${temp_tape}
	@echo vhs $<
	@vhs ${temp_tape} -o $@

asset.%.png: screenshot.%.tape
	@echo "Output ${temp_out}"    >  ${temp_tape}
	@cat  ${prefix_file}          >> ${temp_tape}
	@cat  $<                      >> ${temp_tape}
	@echo "Show"                  >> ${temp_tape}
	@echo "Screenshot $@"         >> ${temp_tape}
	@echo "Sleep 100ms"           >> ${temp_tape}
	@echo vhs $<
	@vhs ${temp_tape} -o ${temp_out}

clean:
	-rm $(recs) $(screenshots)
	-rm ${temp_tape} ${temp_out}

.PHONY: all clean
