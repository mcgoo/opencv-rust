import sys, re, os.path
import logging
from pprint import pformat
from string import Template

if sys.version_info[0] >= 3:
    from io import StringIO
else:
    from cStringIO import StringIO

#
#       EXCEPTIONS TO AUTO GENERATION
#

ManualFuncs = { }

class_ignore_list = (
    #core
    "Algorithm", "FileNode", "FileStorage", "KDTree", "KeyPoint", "DMatch",
    #videoio
#    "VideoWriter",
)

func_arg_fix = {
}

#
#       TYPES MAPPING
#

type_mapping = {
    u"void"  : { u"ctype": "void", "rtype": "()" },
    u"bool"  : { u"ctype": "int", u"rtype": "bool" },
    u"int"   : { u"ctype": "int", u"rtype": "u32" },
    u"int64" : { u"ctype": "int64", u"rtype": "i64" },
    u"float" : { u"ctype": "float", u"rtype": "f32" },
    u"double": { u"ctype": "double", u"rtype": "f64" },
    u"string": { u"ctype": "const char *", u"rtype": "*const i8",
                 u"return_cpp_to_c": Template("return strdup($src.c_str());\n") },
}

boxed_classes = [ "Mat", "Subdiv2D" ]
forced_boxed_classes = {
    "core": [ "Mat" ]
}

#
#       TEMPLATES
#

T_CPP_MODULE = """
//
// This file is auto-generated, please don't edit!
//

#define LOG_TAG "org.opencv.$m"

#include "common.h"
#include <iostream>

#include "opencv2/opencv_modules.hpp"
#ifdef HAVE_OPENCV_$M

#include <string>

#include "opencv2/$m/$m.hpp"
using namespace cv;

typedef struct CScalar { double a; double b; double c; double d; } CScalar;
typedef struct CPoint { int x; int y; } CPoint;

$includes

extern "C" {

$code

} // extern "C"

#endif // HAVE_OPENCV_$M
"""

T_RUST_MODULE = """
//
// This file is auto-generated, please don't edit!
//


use $m::*;

extern "C" {

$externs

} // extern "C"

mod $m {
    $code
}

"""

#
#       AST-LIKE
#

class GeneralInfo():
    def __init__(self, name, namespaces):
        self.namespace, self.classpath, self.classname, self.name = self.parseName(name, namespaces)

    def parseName(self, name, namespaces):
        '''
        input: full name and available namespaces
        returns: (namespace, classpath, classname, name)
        '''
        name = name[name.find(" ")+1:].strip() # remove struct/class/const prefix
        spaceName = ""
        localName = name # <classes>.<name>
        for namespace in sorted(namespaces, key=len, reverse=True):
            if name.startswith(namespace + "."):
                spaceName = namespace
                localName = name.replace(namespace + ".", "")
                break
        pieces = localName.split(".")
        if len(pieces) > 2: # <class>.<class>.<class>.<name>
            return spaceName, ".".join(pieces[:-1]), pieces[-2], pieces[-1]
        elif len(pieces) == 2: # <class>.<name>
            return spaceName, pieces[0], pieces[0], pieces[1]
        elif len(pieces) == 1: # <name>
            return spaceName, "", "", pieces[0]
        else:
            return spaceName, "", "" # error?!

    def fullName(self, isCPP=False):
        result = ".".join([self.fullClass(), self.name])
        return result if not isCPP else result.replace(".", "::")

    def fullClass(self, isCPP=False):
        result = ".".join([f for f in [self.namespace] + self.classpath.split(".") if len(f)>0])
        return result if not isCPP else result.replace(".", "::")

class ArgInfo():
    def __init__(self, arg_tuple): # [ ctype, name, def val, [mod], argno ]
        self.pointer = False
        cpptype = arg_tuple[0]
        if cpptype.endswith("*"):
            cpptype = cpptype[:-1]
            self.pointer = True
#        if cpptype == 'vector_Point2d':
#            cpptype = 'vector_Point2f'
#        elif cpptype == 'vector_Point3d':
#            cpptype = 'vector_Point3f'
        self.cpptype = cpptype
        self.name = arg_tuple[1]
        self.defval = arg_tuple[2]
        self.out = ""
        if "/O" in arg_tuple[3]:
            self.out = "O"
        if "/IO" in arg_tuple[3]:
            self.out = "IO"

    def __repr__(self):
        return Template("ARG $ctype$p $name=$defval").substitute(ctype=self.cpptype,
                                                                  p=" *" if self.pointer else "",
                                                                  name=self.name,
                                                                  defval=self.defval)

class FuncInfo(GeneralInfo):
    def __init__(self, decl, namespaces=[]): # [ funcname, return_ctype, [modifiers], [args] ]
        GeneralInfo.__init__(self, decl[0], namespaces)
        self.cppname = self.name.replace(".", "::")
#        self.jname = self.name
        self.isconstructor = self.name == self.classname
#        if "[" in self.name:
#            self.jname = "getelem"
#        for m in decl[2]:
#            if m.startswith("="):
#                self.jname = m[1:]
        self.static = ["","static"][ "/S" in decl[2] ]
        self.cpptype = re.sub(r"^CvTermCriteria", "TermCriteria", decl[1] or "")
        self.args = []
#        func_fix_map = func_arg_fix.get(self.classname, {}).get(self.jname, {})
        for a in decl[3]:
#            arg = a[:]
#            arg_fix_map = func_fix_map.get(arg[1], {})
#            arg[0] = arg_fix_map.get('ctype',  arg[0]) #fixing arg type
#            arg[3] = arg_fix_map.get('attrib', arg[3]) #fixing arg attrib
            self.args.append(ArgInfo(a))

    def __repr__(self):
        return Template("FUNC <$cpptype $namespace.$classpath.$name $args>").substitute(**self.__dict__)


class ClassInfo(GeneralInfo):
    def __init__(self, decl, namespaces=[]): # [ 'class/struct cname', ': base', [modlist] ]
        GeneralInfo.__init__(self, decl[0], namespaces)
        self.cname = self.name.replace(".", "::")
        self.methods = []
#        self.methods_suffixes = {}
#        self.consts = [] # using a list to save the occurence order
#        self.private_consts = []
#        self.imports = set()
#        self.props= []
#        self.jname = self.name
#        self.j_code = None # java code stream
#        self.jn_code = None # jni code stream
#        self.cpp_code = None # cpp code stream
#        for m in decl[2]:
#            if m.startswith("="):
#                self.jname = m[1:]
#        self.base = ''
#        if decl[1]:
            #self.base = re.sub(r"\b"+self.jname+r"\b", "", decl[1].replace(":", "")).strip()
#            self.base = re.sub(r"^.*:", "", decl[1].split(",")[0]).strip().replace(self.jname, "")

    def __repr__(self):
        return Template("CLASS $namespace.$classpath.$name : $base").substitute(**self.__dict__)

    def addMethod(self, fi):
        self.methods.append(fi)

    def getAllMethods(self):
        result = []
        result.extend([fi for fi in sorted(self.methods) if fi.isconstructor])
        result.extend([fi for fi in sorted(self.methods) if not fi.isconstructor])
        return result
#
#       GENERATOR
#

class RustWrapperGenerator(object):
    def __init__(self):
        self.clear()

    def clear(self):
        self.module = ""
        self.Module = ""
        self.classes = { }
        self.functions = [];
#        self.classes = { "Mat" : ClassInfo([ 'class Mat', '', [], [] ], self.namespaces) }
        self.ported_func_list = []
        self.skipped_func_list = []
        self.def_args_hist = {} # { def_args_cnt : funcs_cnt }

    def getClass(self, classname):
        return self.classes[classname] # or self.Module]

#    def isWrapped(self, classname):
#        name = classname or self.Module
#        return name in self.classes

    def add_class(self, decl):
        classinfo = ClassInfo(decl, namespaces=self.namespaces)
        name = classinfo.name
        if not name in class_ignore_list:
            self.classes[name] = classinfo

    def add_const(self, decl): # [ "const cname", val, [], [] ]
        pass

    def add_func(self, decl):
        fi = FuncInfo(decl, namespaces=self.namespaces)
        if fi.classname == "":
            self.functions.append(fi)
        elif fi.classname in class_ignore_list:
            logging.info('ignored: %s', fi)
        elif fi.classname in ManualFuncs and fi.jname in ManualFuncs[classname]:
            logging.info('manual: %s', fi)
#        elif not self.isWrapped(fi.classname):
#            logging.info('not wrapped: %s', fi)
        elif fi.classname in class_ignore_list:
            pass
        else:
            self.getClass(fi.classname).addMethod(fi)
            # calc args with def val
            cnt = len([a for a in fi.args if a.defval])
            self.def_args_hist[cnt] = self.def_args_hist.get(cnt, 0) + 1

    def save(self, path, buf):
        f = open(path, "wt")
        f.write(buf)
        f.close()

    def gen(self, srcfiles, module, output_path):
        parser = hdr_parser.CppHeaderParser()
        self.module = module
        self.Module = module.capitalize()
        includes = [];

        for hdr in srcfiles:
            decls = parser.parse(hdr)
            self.namespaces = parser.namespaces
            logging.info("\n\n===== Header: %s =====", hdr)
            logging.info("Namespaces: %s", parser.namespaces)
            if decls:
                includes.append('#include "' + hdr + '"')
            for decl in decls:
                logging.info("\n--- Incoming ---\n%s", pformat(decl, 4))
                name = decl[0]
                if name.startswith("struct") or name.startswith("class"):
                    self.add_class(decl)
                elif name.startswith("const"):
                    self.add_const(decl)
                else: # function
                    self.add_func(decl)

        logging.info("\n\n===== Generating... =====")
        self.moduleCppCode = StringIO()
        self.moduleRustCode = StringIO()
        self.moduleRustExterns = StringIO()

        for fi in self.functions:
            self.gen_func(None, fi)

        if module in forced_boxed_classes:
            for cb in forced_boxed_classes[module]:
                self.gen_boxed_class(cb)

        for ci in self.classes.values():
#            if ci.name == "Mat":
#                continue
#            ci.initCodeStreams(self.Module)
            self.gen_class(ci)
#            classJavaCode = ci.generateJavaCode(self.module, self.Module)
#            self.save("%s/%s+%s.java" % (output_path, module, ci.jname), classJavaCode)
#            self.moduleCppCode.write(ci.generateCppCode())
#            ci.cleanupCodeStreams(
#)

        self.save(output_path+"/"+module+".cpp", Template(T_CPP_MODULE).substitute(m = module, M = module.upper(), code = self.moduleCppCode.getvalue(), includes = "\n".join(includes)))
        self.save(output_path+"/"+module+".rs", Template(T_RUST_MODULE).substitute(m = module, M = module.upper(), code = self.moduleRustCode.getvalue(), externs=self.moduleRustExterns.getvalue()))
        self.save(output_path+"/"+module+".txt", self.makeReport())

    def makeReport(self):
        '''
        Returns string with generator report
        '''
        report = StringIO()
        total_count = len(self.ported_func_list)+ len(self.skipped_func_list)
        report.write("PORTED FUNCs LIST (%i of %i):\n\n" % (len(self.ported_func_list), total_count))
        report.write("\n".join(self.ported_func_list))
        report.write("\n\nSKIPPED FUNCs LIST (%i of %i):\n\n" % (len(self.skipped_func_list), total_count))
        report.write("".join(self.skipped_func_list))
        for i in self.def_args_hist.keys():
            report.write("\n%i def args - %i funcs" % (i, self.def_args_hist[i]))
        return report.getvalue()

    def is_boxed(self, type_name):
        return type_name in boxed_classes

    def is_mapped(self, type_name):
        return self.is_boxed(type_name) or type_name in type_mapping

    def map_type(self, type_name):
        if self.is_boxed(type_name):
            return { "ctype" : "void*", "rtype": "%s"%(type_name) }
        else:
            return type_mapping[type_name]

    def gen_func(self, ci, fi, prop_name=''):
        if fi.isconstructor:
            rv_cpptype = ci.name;
        else:
            rv_cpptype = fi.cpptype;
        if not ci == None and not self.is_mapped(ci.name):
            msg = "unmapped class %s "%(ci.name)
            self.skipped_func_list.append("%s\n   %s"%(fi,msg))
            return
        if fi.cppname == "()":
            msg = "can not map operator() yet"
            self.skipped_func_list.append("%s\n   %s"%(fi,msg))
            return
        if not self.is_mapped(rv_cpptype):
            msg = "can not map return value %s \n"%(rv_cpptype)
            self.skipped_func_list.append("%s\n   %s"%(fi,msg))
            return
        for a in fi.args:
            if not self.is_mapped(a.cpptype):
                msg = "can not map arg [%s]\n"%(a)
                self.skipped_func_list.append("%s\n   %s"%(fi,msg))
                return

        rv = self.map_type(rv_cpptype)

        self.ported_func_list.append(fi.__repr__())

        self.moduleCppCode.write("// %s\n"%(fi.cppname))
        self.moduleCppCode.write("// %s\n"%(fi))

        decl_c_args = ""
        call_cpp_args = ""
        decl_rust_extern_args = ""
        suffix = "_" if len(fi.args) > 0 else ""
        if not ci == None and not fi.isconstructor and self.is_boxed(ci.name):
            decl_c_args += ci.name + " *instance"
        for a in fi.args:
            atype = self.map_type(a.cpptype)
            if not decl_c_args == "":
                decl_c_args+=", "
            if not call_cpp_args == "":
                call_cpp_args += ", "
                decl_rust_extern_args += ", "
            suffix += a.cpptype[0].capitalize()
            decl_c_args += atype["ctype"] + " " + a.name
            if self.is_boxed(a.cpptype):
                call_cpp_args += "*((%s*)%s)"%(a.cpptype, a.name)
            elif "arg_c_to_cpp" in atype:
                call_cpp_args += atype["arg_c_to_cpp"].substitute(src=a.name)
            else:
                call_cpp_args += a.name
            rsname = a.name
            if rsname in ["type"]:
                rsname = "_" + rsname
            decl_rust_extern_args += rsname + ": " + atype["rtype"]

        if ci == None:
            c_name = "cv_%s_%s%s"%(module, fi.cppname, suffix);
        else:
            c_name = "cv_%s_%s_%s%s"%(module, ci.name, fi.cppname, suffix);

        # C function prototype
        if self.is_boxed(fi.cpptype):
            self.moduleCppCode.write("%s* %s(%s) {\n"%(fi.cpptype, c_name, decl_c_args));
        else:
            self.moduleCppCode.write("%s %s(%s) {\n"%(rv["ctype"], c_name, decl_c_args));

        # cpp call with prefix
        if ci == None:
            call_name = "cv::" + fi.cppname
        elif fi.isconstructor and self.is_boxed(ci.name):
            call_name = ci.name
        else:
            call_name = "instance->" + fi.cppname

        # return value
        if fi.cpptype == "void":
            self.moduleCppCode.write("  %s(%s);\n"%(call_name, call_cpp_args));
        else:
            self.moduleCppCode.write("  %s cpp_return_value = %s(%s);\n"%(rv_cpptype, call_name,
                call_cpp_args));
            if self.is_boxed(rv_cpptype):
                self.moduleCppCode.write(" return new %s(cpp_return_value);\n"%(rv_cpptype));
            elif "return_cpp_to_c" in rv:
                self.moduleCppCode.write(rv["return_cpp_to_c"].substitute(src="cpp_return_value"));
            else:
                self.moduleCppCode.write("  return cpp_return_value;\n");
        self.moduleCppCode.write("}\n\n");

        self.moduleRustExterns.write("pub fn %s(%s) -> %s;\n"%(c_name, decl_rust_extern_args, rv["rtype"]));

    def gen_boxed_class(self, name):
        self.moduleRustExterns.write("pub fn cv_delete_%s(ptr : *mut i8);\n"%(name));

        self.moduleRustCode.write("#[repr(C)] pub struct %s { ptr: *mut i8 }\n"%(name));
        self.moduleRustCode.write("impl Drop for %s {\n"%(name));
        self.moduleRustCode.write("  fn drop(&mut self) { unsafe { ::cv_delete_%s(self.ptr) }; }\n"%(name));
        self.moduleRustCode.write("}\n")

        self.moduleCppCode.write("void cv_delete_%s(void* instance) {\n"%(name));
        self.moduleCppCode.write("  delete (cv::%s*) instance;\n"%(name));
        self.moduleCppCode.write("}\n");

    def gen_class(self, ci):
        if self.is_boxed(ci.name):
            self.gen_boxed_class(ci.name)
        for fi in ci.getAllMethods():
            self.gen_func(ci, fi)

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage:\n", \
            os.path.basename(sys.argv[0]), \
            "<full path to hdr_parser.py> <out_dir> <module name> <C++ header> [<C++ header>...]")
        print("Current args are: ", ", ".join(["'"+a+"'" for a in sys.argv]))
        exit(0)

    hdr_parser_path = os.path.abspath(sys.argv[1])
    if hdr_parser_path.endswith(".py"):
        hdr_parser_path = os.path.dirname(hdr_parser_path)
    sys.path.append(hdr_parser_path)
    import hdr_parser
    dstdir = sys.argv[2]
    module = sys.argv[3]
    srcfiles = sys.argv[4:]
    logging.basicConfig(filename='%s/%s.log' % (dstdir, module), format=None, filemode='w', level=logging.INFO)
    handler = logging.StreamHandler()
    handler.setLevel(logging.WARNING)
    logging.getLogger().addHandler(handler)
    print("Generating module '" + module + "' from headers:\n\t" + "\n\t".join(srcfiles))
    generator = RustWrapperGenerator()
    generator.gen(srcfiles, module, dstdir)
