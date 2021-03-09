# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: nieyraud <nieyraud@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2019/11/10 16:28:15 by nieyraud          #+#    #+#              #
#    Updated: 2021/03/09 09:16:41 by nieyraud         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

#####################
#		COLOR		#
#####################

rose=\033[1;31m
violetfonce=\033[0;35m
violetclair=\033[1;35m
neutre=\033[0m
cyanfonce=\033[0;36m
cyanclair=\033[1;36m
vertfonce=\033[0;32m
vertclair=\033[1;32m
rouge=\033[31m
orange=\033[33m

#####################
#		SOURCES		#
#####################

NAME = a.out

include files.mk

CLASSES_SRCS= $(addprefix $(CLASSES), .cpp)
CLASSES_HEADERS= $(addprefix $(CLASSES), .hpp) $(INTERFACES)
INCLUDES_DIR = include/
TEMPLATE_DIR = $(addprefix $(INCLUDES_DIR), template/)
CLASSES_DIR = $(addprefix $(INCLUDES_DIR), classes/)

INCLUDE	=	$(addprefix $(TEMPLATE_DIR), $(TEMPLATES)) \
			$(addprefix $(CLASSES_DIR), $(CLASSES_HEADERS)) \
			$(addprefix $(INCLUDES_DIR), $(HEADERS))

#############################
#		DIRCTORIES PATH		#
#############################

PATH = ${shell find srcs -type d}
OBJ_PATH = objs
vpath %.cpp $(foreach dir, $(PATH), $(dir))

#################################
#		FILES	VARIABLE		#
#################################

SRC		= ${SRC_FILE} $(CLASSES_SRCS)
OBJ		= ${addprefix ${OBJ_PATH}/, ${SRC:%.cpp=%.o}}

#########################
#			FLAGS		#
#########################

STD_VERSION= -std=c++98
FLAGS = -Wall -Wextra -ferror-limit=5 $(STD_VERSION)
SAN = -g
OPT_FLAGS = -flto
INCLUDE_FLAGS = -I $(TEMPLATE_DIR) -I $(INCLUDES_DIR) -I $(CLASSES_DIR)

#########################
#		LIBRARIES		#
#########################

LIB_DIR = lib/
LINK_FLAGS = $(addprefix -L, $(LIB_DIR))
LIBS = ${addprefix -l, ${LIBS_BIN}}

########################
#		COMMAND			#
#######################

CC= /usr/bin/clang++
DIFF = /usr/bin/diff
MKDIR= /bin/mkdir
ECHO=echo
RM=/bin/rm

#############################
#			RULES			#
#############################

all : $(NAME)

$(NAME) : ${INCLUDE} ${OBJ} 
	@$(ECHO) "${vertclair}Creating ${NAME}"
	@$(CC) ${FLAGS} ${OPT_FLAGS} $(INCLUDE_FLAGS) ${OBJ} $(LINK_FLAGS) $(LIBS) -o ${NAME}
	@$(ECHO) "${vertclair}[$(NAME) ready to use]$(neutre)"

${OBJ_PATH}/%.o: %.cpp ${INCLUDE}
	@$(MKDIR) -p ${OBJ_PATH}
	@$(ECHO) "${cyanfonce}Compiling ${notdir $(basename $@)}"
	@$(CC) $(FLAGS) -c -o $@ $(INCLUDE_FLAGS) $<

debug: extend_flags re

extend_flags:
	$(eval FLAGS += $(SAN))

clean :
	@$(ECHO) "${rouge}Removing objects files"
	@$(RM) -rf ${OBJ_PATH}

fclean : clean
	@$(ECHO) "${rose}Removing ${NAME}"
	@$(RM) -f $(NAME)

re : fclean all

.PHONY : all clean fclean re f
