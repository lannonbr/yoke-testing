package main

import (
	"encoding/json"
	"fmt"
	"os"

	"flag"

	"github.com/yokecd/yoke/pkg/flight"

	appsv1 "k8s.io/api/apps/v1"
	corev1 "k8s.io/api/core/v1"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/utils/ptr"
)

// Code primarially referenced from the Yoke docs: https://yokecd.github.io/docs/examples/basics/

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func run() error {
	var (
		release      = flight.Release()
		namespace    = flight.Namespace()
		labels       = map[string]string{"app": release}
		replicaCount = flag.Int("r", 1, "how many replicas.")
	)

	flag.Parse()

	resources := []flight.Resource{
		CreateDeployment(DeploymentConfig{
			Name:      "example-deploy",
			Namespace: namespace,
			Labels:    labels,
			Replicas:  int32(*replicaCount),
		}),
	}

	return json.NewEncoder(os.Stdout).Encode(resources)
}

type DeploymentConfig struct {
	Name      string
	Namespace string
	Labels    map[string]string
	Replicas  int32
}

func CreateDeployment(cfg DeploymentConfig) *appsv1.Deployment {
	return &appsv1.Deployment{
		TypeMeta: metav1.TypeMeta{
			APIVersion: appsv1.SchemeGroupVersion.Identifier(),
			Kind:       "Deployment",
		},
		ObjectMeta: metav1.ObjectMeta{
			Name:      cfg.Name,
			Namespace: cfg.Namespace,
		},
		Spec: appsv1.DeploymentSpec{
			Selector: &metav1.LabelSelector{
				MatchLabels: cfg.Labels,
			},
			Replicas: ptr.To(cfg.Replicas),
			Template: corev1.PodTemplateSpec{
				ObjectMeta: metav1.ObjectMeta{
					Labels: cfg.Labels,
				},
				Spec: corev1.PodSpec{
					Containers: []corev1.Container{
						{
							Name:    cfg.Name,
							Image:   "alpine:latest",
							Command: []string{"watch", "echo", "hello world"},
						},
					},
				},
			},
		},
	}
}
