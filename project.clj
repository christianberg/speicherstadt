(defproject speicherstadt "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :min-lein-version "2.0.0"
  :dependencies [[org.clojure/clojure "1.8.0"]
                 [com.stuartsierra/component "0.3.1"]
                 [compojure "1.5.0"]
                 [duct "0.7.0"]
                 [environ "1.0.3"]
                 [meta-merge "0.1.1"]
                 [me.raynes/fs "1.4.6"]
                 [ring "1.4.0"]
                 [ring/ring-defaults "0.2.0"]
                 [ring-jetty-component "0.3.1"]
                 [ring/ring-json "0.4.0"]
                 [com.soundcloud/prometheus-clj "2.4.0"]]
  :plugins [[lein-environ "1.0.3"]]
  :main ^:skip-aot speicherstadt.main
  :target-path "target/%s/"
  :aliases {"run-task" ["with-profile" "+repl" "run" "-m"]
            "setup"    ["run-task" "dev.tasks/setup"]}
  :profiles
  {:dev  [:project/dev  :profiles/dev]
   :test [:project/test :profiles/test]
   :uberjar {:aot :all}
   :profiles/dev  {}
   :profiles/test {}
   :project/dev   {:dependencies [[duct/generate "0.7.0"]
                                  [reloaded.repl "0.2.2"]
                                  [org.clojure/tools.namespace "0.2.11"]
                                  [org.clojure/tools.nrepl "0.2.12"]
                                  [eftest "0.1.1"]
                                  [kerodon "0.7.0"]]
                   :source-paths ["dev"]
                   :repl-options {:init-ns user}
                   :env {:port "3000"}}
   :project/test  {:dependencies [[ring/ring-mock "0.3.0"]]}})
