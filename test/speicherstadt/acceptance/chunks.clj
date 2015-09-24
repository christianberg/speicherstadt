(ns speicherstadt.acceptance.chunks
  (:require [speicherstadt.system :as system]
            [com.stuartsierra.component :as component]
            [clojure.test :refer :all]))

(deftest chunk-acceptance
  (let [sys (component/start (system/new-system {:http {:port 3333}}))]
    (let [handler (-> sys :app :handler)]
      (is (= (-> {:uri "/chunks/dead-beef"
                  :request-method :put}
                 handler
                 :status)
             204)))))
