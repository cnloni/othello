PROJECT := main
THEADERS := tool.hpp nodelet.hpp board.hpp nodeman.hpp bitop.hpp b36.hpp b16.hpp
BUILD_MODE := run
THISDIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

SDIR := $(THISDIR)src
PROJECT_ROOT := $(THISDIR:/=)
BDIR := $(abspath $(PROJECT_ROOT)/build)

CXXFLAGS := -I. -O3 -Wall -std=c++17 -Wno-maybe-uninitialized
LDFLAGS := -L/usr/lib -lpthread -ldl -lutil -lm

.PHONY:	exec test clean $(BDIR)

OBJS := $(CPPS:%.cpp=$(BDIR)/%.o) $(LIBCPPS:%.cpp=$(BDIR)/%.o)
HEADERS = $(THEADERS:%=$(SDIR)/%)
TARGET := $(PROJECT:%=$(BDIR)/%.out)
TESTER := $(BDIR)/test.out

all:	$(BDIR) $(TARGET)

$(BDIR):
	mkdir -p $@

ifeq ($(BUILD_MODE),debug)
$(info BUILD_MODE is debug)
CXXFLAGS += -g -DDEBUG
else ifeq ($(BUILD_MODE),run)
$(info BUILD_MODE is run)
CXXFLAGS += -DNDEBUG -ffunction-sections -fdata-sections
LDFLAGS += -Wl,--gc-sections -Wl,--print-gc-sections
else
$(error Build mode "$(BUILD_MODE)" not supported by this Makefile)
endif

$(BDIR)/%.o:	$(SDIR)/%.cpp $(HEADERS)
	$(CXX) -c $(CXXFLAGS) -o $@ $<

$(BDIR)/%.out:	$(BDIR)/%.o $(OBJS)
	$(CXX) $(LDFLAGS) -o $@ $^

exec:	$(BDIR)/$(PROJECT).out
	$< $(ARG)

test:	$(BDIR) $(TESTER)
	$(TESTER)

clean:
	$(RM) $(TARGET) $(TESTER) $(OBJS)
